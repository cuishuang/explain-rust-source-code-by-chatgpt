// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::unsync::spawn_blocking;
use deno_core::unsync::TaskQueue;
use deno_core::AsyncMutFuture;
use deno_core::AsyncRefCell;
use deno_core::AsyncResult;
use deno_core::BufMutView;
use deno_core::BufView;
use deno_core::CancelHandle;
use deno_core::CancelTryFuture;
use deno_core::Op;
use deno_core::OpState;
use deno_core::RcRef;
use deno_core::Resource;
use deno_core::ResourceHandle;
use deno_core::ResourceHandleFd;
use fs::FileResource;
use fs::FsError;
use fs::FsResult;
use fs::FsStat;
use fs3::FileExt;
use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fs::File as StdFile;
use std::future::Future;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::rc::Rc;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::process;

#[cfg(unix)]
use std::os::unix::io::FromRawFd;

#[cfg(windows)]
use std::os::windows::io::FromRawHandle;
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase;

pub mod fs;

// Store the stdio fd/handles in global statics in order to keep them
// alive for the duration of the application since the last handle/fd
// being dropped will close the corresponding pipe.
#[cfg(unix)]
pub static STDIN_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stdin
  unsafe { StdFile::from_raw_fd(0) }
});
#[cfg(unix)]
pub static STDOUT_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stdout
  unsafe { StdFile::from_raw_fd(1) }
});
#[cfg(unix)]
pub static STDERR_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stderr
  unsafe { StdFile::from_raw_fd(2) }
});

#[cfg(windows)]
pub static STDIN_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stdin
  unsafe { StdFile::from_raw_handle(GetStdHandle(winbase::STD_INPUT_HANDLE)) }
});
#[cfg(windows)]
pub static STDOUT_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stdout
  unsafe { StdFile::from_raw_handle(GetStdHandle(winbase::STD_OUTPUT_HANDLE)) }
});
#[cfg(windows)]
pub static STDERR_HANDLE: Lazy<StdFile> = Lazy::new(|| {
  // SAFETY: corresponds to OS stderr
  unsafe { StdFile::from_raw_handle(GetStdHandle(winbase::STD_ERROR_HANDLE)) }
});

deno_core::extension!(deno_io,
  deps = [ deno_web ],
  esm = [ "12_io.js" ],
  options = {
    stdio: Option<Stdio>,
  },
  middleware = |op| match op.name {
    "op_print" => op_print::DECL,
    _ => op,
  },
  state = |state, options| {
    if let Some(stdio) = options.stdio {
      let t = &mut state.resource_table;

      let rid = t.add(fs::FileResource::new(
        Rc::new(match stdio.stdin {
          StdioPipe::Inherit => StdFileResourceInner::new(
            StdFileResourceKind::Stdin,
            STDIN_HANDLE.try_clone().unwrap(),
          ),
          StdioPipe::File(pipe) => StdFileResourceInner::file(pipe),
        }),
        "stdin".to_string(),
      ));
      assert_eq!(rid, 0, "stdin must have ResourceId 0");

      let rid = t.add(FileResource::new(
        Rc::new(match stdio.stdout {
          StdioPipe::Inherit => StdFileResourceInner::new(
            StdFileResourceKind::Stdout,
            STDOUT_HANDLE.try_clone().unwrap(),
          ),
          StdioPipe::File(pipe) => StdFileResourceInner::file(pipe),
        }),
        "stdout".to_string(),
      ));
      assert_eq!(rid, 1, "stdout must have ResourceId 1");

      let rid = t.add(FileResource::new(
        Rc::new(match stdio.stderr {
          StdioPipe::Inherit => StdFileResourceInner::new(
            StdFileResourceKind::Stderr,
            STDERR_HANDLE.try_clone().unwrap(),
          ),
          StdioPipe::File(pipe) => StdFileResourceInner::file(pipe),
        }),
        "stderr".to_string(),
      ));
      assert_eq!(rid, 2, "stderr must have ResourceId 2");
    }
  },
);

pub enum StdioPipe {
  Inherit,
  File(StdFile),
}

impl Default for StdioPipe {
  fn default() -> Self {
    Self::Inherit
  }
}

impl Clone for StdioPipe {
  fn clone(&self) -> Self {
    match self {
      StdioPipe::Inherit => StdioPipe::Inherit,
      StdioPipe::File(pipe) => StdioPipe::File(pipe.try_clone().unwrap()),
    }
  }
}

/// Specify how stdin, stdout, and stderr are piped.
/// By default, inherits from the process.
#[derive(Clone, Default)]
pub struct Stdio {
  pub stdin: StdioPipe,
  pub stdout: StdioPipe,
  pub stderr: StdioPipe,
}

#[derive(Debug)]
pub struct WriteOnlyResource<S> {
  stream: AsyncRefCell<S>,
}

impl<S: 'static> From<S> for WriteOnlyResource<S> {
  fn from(stream: S) -> Self {
    Self {
      stream: stream.into(),
    }
  }
}

impl<S> WriteOnlyResource<S>
where
  S: AsyncWrite + Unpin + 'static,
{
  pub fn borrow_mut(self: &Rc<Self>) -> AsyncMutFuture<S> {
    RcRef::map(self, |r| &r.stream).borrow_mut()
  }

  async fn write(self: Rc<Self>, data: &[u8]) -> Result<usize, AnyError> {
    let mut stream = self.borrow_mut().await;
    let nwritten = stream.write(data).await?;
    Ok(nwritten)
  }

  async fn shutdown(self: Rc<Self>) -> Result<(), AnyError> {
    let mut stream = self.borrow_mut().await;
    stream.shutdown().await?;
    Ok(())
  }

  pub fn into_inner(self) -> S {
    self.stream.into_inner()
  }
}

#[derive(Debug)]
pub struct ReadOnlyResource<S> {
  stream: AsyncRefCell<S>,
  cancel_handle: CancelHandle,
}

impl<S: 'static> From<S> for ReadOnlyResource<S> {
  fn from(stream: S) -> Self {
    Self {
      stream: stream.into(),
      cancel_handle: Default::default(),
    }
  }
}

impl<S> ReadOnlyResource<S>
where
  S: AsyncRead + Unpin + 'static,
{
  pub fn borrow_mut(self: &Rc<Self>) -> AsyncMutFuture<S> {
    RcRef::map(self, |r| &r.stream).borrow_mut()
  }

  pub fn cancel_handle(self: &Rc<Self>) -> RcRef<CancelHandle> {
    RcRef::map(self, |r| &r.cancel_handle)
  }

  pub fn cancel_read_ops(&self) {
    self.cancel_handle.cancel()
  }

  async fn read(self: Rc<Self>, data: &mut [u8]) -> Result<usize, AnyError> {
    let mut rd = self.borrow_mut().await;
    let nread = rd.read(data).try_or_cancel(self.cancel_handle()).await?;
    Ok(nread)
  }

  pub fn into_inner(self) -> S {
    self.stream.into_inner()
  }
}

pub type ChildStdinResource = WriteOnlyResource<process::ChildStdin>;

impl Resource for ChildStdinResource {
  fn name(&self) -> Cow<str> {
    "childStdin".into()
  }

  deno_core::impl_writable!();

  fn shutdown(self: Rc<Self>) -> AsyncResult<()> {
    Box::pin(self.shutdown())
  }
}

pub type ChildStdoutResource = ReadOnlyResource<process::ChildStdout>;

impl Resource for ChildStdoutResource {
  deno_core::impl_readable_byob!();

  fn name(&self) -> Cow<str> {
    "childStdout".into()
  }

  fn close(self: Rc<Self>) {
    self.cancel_read_ops();
  }
}

pub type ChildStderrResource = ReadOnlyResource<process::ChildStderr>;

impl Resource for ChildStderrResource {
  deno_core::impl_readable_byob!();

  fn name(&self) -> Cow<str> {
    "childStderr".into()
  }

  fn close(self: Rc<Self>) {
    self.cancel_read_ops();
  }
}

#[derive(Clone, Copy)]
enum StdFileResourceKind {
  File,
  // For stdout and stderr, we sometimes instead use std::io::stdout() directly,
  // because we get some Windows specific functionality for free by using Rust
  // std's wrappers. So we take a bit of a complexity hit in order to not
  // have to duplicate the functionality in Rust's std/src/sys/windows/stdio.rs
  Stdin,
  Stdout,
  Stderr,
}

pub struct StdFileResourceInner {
  kind: StdFileResourceKind,
  // We can't use an AsyncRefCell here because we need to allow
  // access to the resource synchronously at any time and
  // asynchronously one at a time in order
  cell: RefCell<Option<StdFile>>,
  // Used to keep async actions in order and only allow one
  // to occur at a time
  cell_async_task_queue: Rc<TaskQueue>,
  handle: ResourceHandleFd,
}

impl StdFileResourceInner {
  pub fn file(fs_file: StdFile) -> Self {
    StdFileResourceInner::new(StdFileResourceKind::File, fs_file)
  }

  fn new(kind: StdFileResourceKind, fs_file: StdFile) -> Self {
    // We know this will be an fd
    let handle = ResourceHandle::from_fd_like(&fs_file).as_fd_like().unwrap();
    StdFileResourceInner {
      kind,
      handle,
      cell: RefCell::new(Some(fs_file)),
      cell_async_task_queue: Default::default(),
    }
  }

  fn with_sync<F, R>(&self, action: F) -> FsResult<R>
  where
    F: FnOnce(&mut StdFile) -> FsResult<R>,
  {
    match self.cell.try_borrow_mut() {
      Ok(mut cell) if cell.is_some() => action(cell.as_mut().unwrap()),
      _ => Err(fs::FsError::FileBusy),
    }
  }

  fn with_inner_blocking_task<F, R: 'static + Send>(
    &self,
    action: F,
  ) -> impl Future<Output = R> + '_
  where
    F: FnOnce(&mut StdFile) -> R + Send + 'static,
  {
    // we want to restrict this to one async action at a time
    let acquire_fut = self.cell_async_task_queue.acquire();
    async move {
      let permit = acquire_fut.await;
      // we take the value out of the cell, use it on a blocking task,
      // then put it back into the cell when we're done
      let mut did_take = false;
      let mut cell_value = {
        let mut cell = self.cell.borrow_mut();
        match cell.as_mut().unwrap().try_clone().ok() {
          Some(value) => value,
          None => {
            did_take = true;
            cell.take().unwrap()
          }
        }
      };
      let (cell_value, result) = spawn_blocking(move || {
        let result = action(&mut cell_value);
        (cell_value, result)
      })
      .await
      .unwrap();

      if did_take {
        // put it back
        self.cell.borrow_mut().replace(cell_value);
      }

      drop(permit); // explicit for clarity
      result
    }
  }

  fn with_blocking_task<F, R: 'static + Send>(
    &self,
    action: F,
  ) -> impl Future<Output = R>
  where
    F: FnOnce() -> R + Send + 'static,
  {
    // we want to restrict this to one async action at a time
    let acquire_fut = self.cell_async_task_queue.acquire();
    async move {
      let _permit = acquire_fut.await;
      spawn_blocking(action).await.unwrap()
    }
  }
}

#[async_trait::async_trait(?Send)]
impl crate::fs::File for StdFileResourceInner {
  fn write_sync(self: Rc<Self>, buf: &[u8]) -> FsResult<usize> {
    // Rust will line buffer and we don't want that behavior
    // (see https://github.com/denoland/deno/issues/948), so flush stdout and stderr.
    // Although an alternative solution could be to bypass Rust's std by
    // using the raw fds/handles, it will cause encoding issues on Windows
    // that we get solved for free by using Rust's stdio wrappers (see
    // std/src/sys/windows/stdio.rs in Rust's source code).
    match self.kind {
      StdFileResourceKind::File => self.with_sync(|file| Ok(file.write(buf)?)),
      StdFileResourceKind::Stdin => {
        Err(Into::<std::io::Error>::into(ErrorKind::Unsupported).into())
      }
      StdFileResourceKind::Stdout => {
        // bypass the file and use std::io::stdout()
        let mut stdout = std::io::stdout().lock();
        let nwritten = stdout.write(buf)?;
        stdout.flush()?;
        Ok(nwritten)
      }
      StdFileResourceKind::Stderr => {
        // bypass the file and use std::io::stderr()
        let mut stderr = std::io::stderr().lock();
        let nwritten = stderr.write(buf)?;
        stderr.flush()?;
        Ok(nwritten)
      }
    }
  }

  fn read_sync(self: Rc<Self>, buf: &mut [u8]) -> FsResult<usize> {
    match self.kind {
      StdFileResourceKind::File | StdFileResourceKind::Stdin => {
        self.with_sync(|file| Ok(file.read(buf)?))
      }
      StdFileResourceKind::Stdout | StdFileResourceKind::Stderr => {
        Err(FsError::NotSupported)
      }
    }
  }

  fn write_all_sync(self: Rc<Self>, buf: &[u8]) -> FsResult<()> {
    match self.kind {
      StdFileResourceKind::File => {
        self.with_sync(|file| Ok(file.write_all(buf)?))
      }
      StdFileResourceKind::Stdin => {
        Err(Into::<std::io::Error>::into(ErrorKind::Unsupported).into())
      }
      StdFileResourceKind::Stdout => {
        // bypass the file and use std::io::stdout()
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(buf)?;
        stdout.flush()?;
        Ok(())
      }
      StdFileResourceKind::Stderr => {
        // bypass the file and use std::io::stderr()
        let mut stderr = std::io::stderr().lock();
        stderr.write_all(buf)?;
        stderr.flush()?;
        Ok(())
      }
    }
  }
  async fn write_all(self: Rc<Self>, buf: BufView) -> FsResult<()> {
    match self.kind {
      StdFileResourceKind::File => {
        self
          .with_inner_blocking_task(move |file| Ok(file.write_all(&buf)?))
          .await
      }
      StdFileResourceKind::Stdin => {
        Err(Into::<std::io::Error>::into(ErrorKind::Unsupported).into())
      }
      StdFileResourceKind::Stdout => {
        self
          .with_blocking_task(move || {
            // bypass the file and use std::io::stdout()
            let mut stdout = std::io::stdout().lock();
            stdout.write_all(&buf)?;
            stdout.flush()?;
            Ok(())
          })
          .await
      }
      StdFileResourceKind::Stderr => {
        self
          .with_blocking_task(move || {
            // bypass the file and use std::io::stderr()
            let mut stderr = std::io::stderr().lock();
            stderr.write_all(&buf)?;
            stderr.flush()?;
            Ok(())
          })
          .await
      }
    }
  }

  async fn write(
    self: Rc<Self>,
    view: BufView,
  ) -> FsResult<deno_core::WriteOutcome> {
    match self.kind {
      StdFileResourceKind::File => {
        self
          .with_inner_blocking_task(|file| {
            let nwritten = file.write(&view)?;
            Ok(deno_core::WriteOutcome::Partial { nwritten, view })
          })
          .await
      }
      StdFileResourceKind::Stdin => {
        Err(Into::<std::io::Error>::into(ErrorKind::Unsupported).into())
      }
      StdFileResourceKind::Stdout => {
        self
          .with_blocking_task(|| {
            // bypass the file and use std::io::stdout()
            let mut stdout = std::io::stdout().lock();
            let nwritten = stdout.write(&view)?;
            stdout.flush()?;
            Ok(deno_core::WriteOutcome::Partial { nwritten, view })
          })
          .await
      }
      StdFileResourceKind::Stderr => {
        self
          .with_blocking_task(|| {
            // bypass the file and use std::io::stderr()
            let mut stderr = std::io::stderr().lock();
            let nwritten = stderr.write(&view)?;
            stderr.flush()?;
            Ok(deno_core::WriteOutcome::Partial { nwritten, view })
          })
          .await
      }
    }
  }

  fn read_all_sync(self: Rc<Self>) -> FsResult<Vec<u8>> {
    match self.kind {
      StdFileResourceKind::File | StdFileResourceKind::Stdin => {
        let mut buf = Vec::new();
        self.with_sync(|file| Ok(file.read_to_end(&mut buf)?))?;
        Ok(buf)
      }
      StdFileResourceKind::Stdout | StdFileResourceKind::Stderr => {
        Err(FsError::NotSupported)
      }
    }
  }
  async fn read_all_async(self: Rc<Self>) -> FsResult<Vec<u8>> {
    match self.kind {
      StdFileResourceKind::File | StdFileResourceKind::Stdin => {
        self
          .with_inner_blocking_task(|file| {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            Ok(buf)
          })
          .await
      }
      StdFileResourceKind::Stdout | StdFileResourceKind::Stderr => {
        Err(FsError::NotSupported)
      }
    }
  }

  fn chmod_sync(self: Rc<Self>, _mode: u32) -> FsResult<()> {
    #[cfg(unix)]
    {
      use std::os::unix::prelude::PermissionsExt;
      self.with_sync(|file| {
        Ok(file.set_permissions(std::fs::Permissions::from_mode(_mode))?)
      })
    }
    #[cfg(not(unix))]
    Err(FsError::NotSupported)
  }
  async fn chmod_async(self: Rc<Self>, _mode: u32) -> FsResult<()> {
    #[cfg(unix)]
    {
      use std::os::unix::prelude::PermissionsExt;
      self
        .with_inner_blocking_task(move |file| {
          Ok(file.set_permissions(std::fs::Permissions::from_mode(_mode))?)
        })
        .await
    }
    #[cfg(not(unix))]
    Err(FsError::NotSupported)
  }

  fn seek_sync(self: Rc<Self>, pos: io::SeekFrom) -> FsResult<u64> {
    self.with_sync(|file| Ok(file.seek(pos)?))
  }
  async fn seek_async(self: Rc<Self>, pos: io::SeekFrom) -> FsResult<u64> {
    self
      .with_inner_blocking_task(move |file| Ok(file.seek(pos)?))
      .await
  }

  fn datasync_sync(self: Rc<Self>) -> FsResult<()> {
    self.with_sync(|file| Ok(file.sync_data()?))
  }
  async fn datasync_async(self: Rc<Self>) -> FsResult<()> {
    self
      .with_inner_blocking_task(|file| Ok(file.sync_data()?))
      .await
  }

  fn sync_sync(self: Rc<Self>) -> FsResult<()> {
    self.with_sync(|file| Ok(file.sync_all()?))
  }
  async fn sync_async(self: Rc<Self>) -> FsResult<()> {
    self
      .with_inner_blocking_task(|file| Ok(file.sync_all()?))
      .await
  }

  fn stat_sync(self: Rc<Self>) -> FsResult<FsStat> {
    self.with_sync(|file| Ok(file.metadata().map(FsStat::from_std)?))
  }
  async fn stat_async(self: Rc<Self>) -> FsResult<FsStat> {
    self
      .with_inner_blocking_task(|file| {
        Ok(file.metadata().map(FsStat::from_std)?)
      })
      .await
  }

  fn lock_sync(self: Rc<Self>, exclusive: bool) -> FsResult<()> {
    self.with_sync(|file| {
      if exclusive {
        file.lock_exclusive()?;
      } else {
        file.lock_shared()?;
      }
      Ok(())
    })
  }
  async fn lock_async(self: Rc<Self>, exclusive: bool) -> FsResult<()> {
    self
      .with_inner_blocking_task(move |file| {
        if exclusive {
          file.lock_exclusive()?;
        } else {
          file.lock_shared()?;
        }
        Ok(())
      })
      .await
  }

  fn unlock_sync(self: Rc<Self>) -> FsResult<()> {
    self.with_sync(|file| Ok(file.unlock()?))
  }
  async fn unlock_async(self: Rc<Self>) -> FsResult<()> {
    self
      .with_inner_blocking_task(|file| Ok(file.unlock()?))
      .await
  }

  fn truncate_sync(self: Rc<Self>, len: u64) -> FsResult<()> {
    self.with_sync(|file| Ok(file.set_len(len)?))
  }
  async fn truncate_async(self: Rc<Self>, len: u64) -> FsResult<()> {
    self
      .with_inner_blocking_task(move |file| Ok(file.set_len(len)?))
      .await
  }

  fn utime_sync(
    self: Rc<Self>,
    atime_secs: i64,
    atime_nanos: u32,
    mtime_secs: i64,
    mtime_nanos: u32,
  ) -> FsResult<()> {
    let atime = filetime::FileTime::from_unix_time(atime_secs, atime_nanos);
    let mtime = filetime::FileTime::from_unix_time(mtime_secs, mtime_nanos);

    self.with_sync(|file| {
      filetime::set_file_handle_times(file, Some(atime), Some(mtime))?;
      Ok(())
    })
  }
  async fn utime_async(
    self: Rc<Self>,
    atime_secs: i64,
    atime_nanos: u32,
    mtime_secs: i64,
    mtime_nanos: u32,
  ) -> FsResult<()> {
    let atime = filetime::FileTime::from_unix_time(atime_secs, atime_nanos);
    let mtime = filetime::FileTime::from_unix_time(mtime_secs, mtime_nanos);

    self
      .with_inner_blocking_task(move |file| {
        filetime::set_file_handle_times(file, Some(atime), Some(mtime))?;
        Ok(())
      })
      .await
  }

  async fn read_byob(
    self: Rc<Self>,
    mut buf: BufMutView,
  ) -> FsResult<(usize, BufMutView)> {
    self
      .with_inner_blocking_task(|file| {
        let nread = file.read(&mut buf)?;
        Ok((nread, buf))
      })
      .await
  }

  fn try_clone_inner(self: Rc<Self>) -> FsResult<Rc<dyn fs::File>> {
    let inner: &Option<_> = &self.cell.borrow();
    match inner {
      Some(inner) => Ok(Rc::new(StdFileResourceInner {
        kind: self.kind,
        cell: RefCell::new(Some(inner.try_clone()?)),
        cell_async_task_queue: Default::default(),
        handle: self.handle,
      })),
      None => Err(FsError::FileBusy),
    }
  }

  fn as_stdio(self: Rc<Self>) -> FsResult<std::process::Stdio> {
    match self.kind {
      StdFileResourceKind::File => self.with_sync(|file| {
        let file = file.try_clone()?;
        Ok(file.into())
      }),
      _ => Ok(std::process::Stdio::inherit()),
    }
  }

  fn backing_fd(self: Rc<Self>) -> Option<ResourceHandleFd> {
    Some(self.handle)
  }
}

// override op_print to use the stdout and stderr in the resource table
#[op2(fast)]
pub fn op_print(
  state: &mut OpState,
  #[string] msg: &str,
  is_err: bool,
) -> Result<(), AnyError> {
  let rid = if is_err { 2 } else { 1 };
  FileResource::with_file(state, rid, move |file| {
    Ok(file.write_all_sync(msg.as_bytes())?)
  })
}
