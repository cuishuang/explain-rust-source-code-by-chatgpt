// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use std::borrow::Cow;
use std::cell::RefCell;
use std::io;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;

use deno_core::error::custom_error;
use deno_core::error::type_error;
use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::CancelFuture;
use deno_core::CancelHandle;
use deno_core::JsBuffer;
use deno_core::OpState;
use deno_core::ResourceId;
use deno_core::ToJsBuffer;
use deno_io::fs::FileResource;
use deno_io::fs::FsError;
use deno_io::fs::FsStat;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;
use serde::Serialize;

use crate::check_unstable;
use crate::interface::FileSystemRc;
use crate::interface::FsDirEntry;
use crate::interface::FsFileType;
use crate::FsPermissions;
use crate::OpenOptions;

#[op2]
#[string]
pub fn op_fs_cwd<P>(state: &mut OpState) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let fs = state.borrow::<FileSystemRc>();
  let path = fs.cwd()?;
  state
    .borrow_mut::<P>()
    .check_read_blind(&path, "CWD", "Deno.cwd()")?;
  let path_str = path_into_string(path.into_os_string())?;
  Ok(path_str)
}

#[op2(fast)]
pub fn op_fs_chdir<P>(
  state: &mut OpState,
  #[string] directory: &str,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let d = PathBuf::from(&directory);
  state.borrow_mut::<P>().check_read(&d, "Deno.chdir()")?;
  state
    .borrow::<FileSystemRc>()
    .chdir(&d)
    .context_path("chdir", &d)
}

#[op2]
pub fn op_fs_umask(
  state: &mut OpState,
  mask: Option<u32>,
) -> Result<u32, AnyError>
where
{
  check_unstable(state, "Deno.umask");
  state.borrow::<FileSystemRc>().umask(mask).context("umask")
}

#[op2]
#[smi]
pub fn op_fs_open_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  #[serde] options: Option<OpenOptions>,
) -> Result<ResourceId, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let options = options.unwrap_or_else(OpenOptions::read);
  let permissions = state.borrow_mut::<P>();
  permissions.check(&options, &path, "Deno.openSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  let file = fs.open_sync(&path, options).context_path("open", &path)?;

  let rid = state
    .resource_table
    .add(FileResource::new(file, "fsFile".to_string()));
  Ok(rid)
}

#[op2(async)]
#[smi]
pub async fn op_fs_open_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[serde] options: Option<OpenOptions>,
) -> Result<ResourceId, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let options = options.unwrap_or_else(OpenOptions::read);
  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check(&options, &path, "Deno.open()")?;
    state.borrow::<FileSystemRc>().clone()
  };
  let file = fs
    .open_async(path.clone(), options)
    .await
    .context_path("open", &path)?;

  let rid = state
    .borrow_mut()
    .resource_table
    .add(FileResource::new(file, "fsFile".to_string()));
  Ok(rid)
}

#[op2]
pub fn op_fs_mkdir_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  recursive: bool,
  mode: Option<u32>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let mode = mode.unwrap_or(0o777) & 0o777;

  state
    .borrow_mut::<P>()
    .check_write(&path, "Deno.mkdirSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.mkdir_sync(&path, recursive, mode)
    .context_path("mkdir", &path)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_mkdir_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  recursive: bool,
  mode: Option<u32>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let mode = mode.unwrap_or(0o777) & 0o777;

  let fs = {
    let mut state = state.borrow_mut();
    state.borrow_mut::<P>().check_write(&path, "Deno.mkdir()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.mkdir_async(path.clone(), recursive, mode)
    .await
    .context_path("mkdir", &path)?;

  Ok(())
}

#[op2(fast)]
pub fn op_fs_chmod_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  mode: u32,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  state
    .borrow_mut::<P>()
    .check_write(&path, "Deno.chmodSync()")?;
  let fs = state.borrow::<FileSystemRc>();
  fs.chmod_sync(&path, mode).context_path("chmod", &path)?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_chmod_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  mode: u32,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  let fs = {
    let mut state = state.borrow_mut();
    state.borrow_mut::<P>().check_write(&path, "Deno.chmod()")?;
    state.borrow::<FileSystemRc>().clone()
  };
  fs.chmod_async(path.clone(), mode)
    .await
    .context_path("chmod", &path)?;
  Ok(())
}

#[op2]
pub fn op_fs_chown_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  uid: Option<u32>,
  gid: Option<u32>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  state
    .borrow_mut::<P>()
    .check_write(&path, "Deno.chownSync()")?;
  let fs = state.borrow::<FileSystemRc>();
  fs.chown_sync(&path, uid, gid)
    .context_path("chown", &path)?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_chown_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  uid: Option<u32>,
  gid: Option<u32>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  let fs = {
    let mut state = state.borrow_mut();
    state.borrow_mut::<P>().check_write(&path, "Deno.chown()")?;
    state.borrow::<FileSystemRc>().clone()
  };
  fs.chown_async(path.clone(), uid, gid)
    .await
    .context_path("chown", &path)?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_remove_sync<P>(
  state: &mut OpState,
  #[string] path: &str,
  recursive: bool,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  state
    .borrow_mut::<P>()
    .check_write(&path, "Deno.removeSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.remove_sync(&path, recursive)
    .context_path("remove", &path)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_remove_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  recursive: bool,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = {
    let mut state = state.borrow_mut();
    if recursive {
      state
        .borrow_mut::<P>()
        .check_write(&path, "Deno.remove()")?;
    } else {
      state
        .borrow_mut::<P>()
        .check_write_partial(&path, "Deno.remove()")?;
    }

    state.borrow::<FileSystemRc>().clone()
  };

  fs.remove_async(path.clone(), recursive)
    .await
    .context_path("remove", &path)?;

  Ok(())
}

#[op2(fast)]
pub fn op_fs_copy_file_sync<P>(
  state: &mut OpState,
  #[string] from: &str,
  #[string] to: &str,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let from = PathBuf::from(from);
  let to = PathBuf::from(to);

  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&from, "Deno.copyFileSync()")?;
  permissions.check_write(&to, "Deno.copyFileSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.copy_file_sync(&from, &to)
    .context_two_path("copy", &from, &to)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_copy_file_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] from: String,
  #[string] to: String,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let from = PathBuf::from(from);
  let to = PathBuf::from(to);

  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&from, "Deno.copyFile()")?;
    permissions.check_write(&to, "Deno.copyFile()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.copy_file_async(from.clone(), to.clone())
    .await
    .context_two_path("copy", &from, &to)?;

  Ok(())
}

#[op2(fast)]
pub fn op_fs_stat_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  #[buffer] stat_out_buf: &mut [u32],
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  state
    .borrow_mut::<P>()
    .check_read(&path, "Deno.statSync()")?;
  let fs = state.borrow::<FileSystemRc>();
  let stat = fs.stat_sync(&path).context_path("stat", &path)?;
  let serializable_stat = SerializableStat::from(stat);
  serializable_stat.write(stat_out_buf);
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_fs_stat_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
) -> Result<SerializableStat, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&path, "Deno.stat()")?;
    state.borrow::<FileSystemRc>().clone()
  };
  let stat = fs
    .stat_async(path.clone())
    .await
    .context_path("stat", &path)?;
  Ok(SerializableStat::from(stat))
}

#[op2(fast)]
pub fn op_fs_lstat_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  #[buffer] stat_out_buf: &mut [u32],
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  state
    .borrow_mut::<P>()
    .check_read(&path, "Deno.lstatSync()")?;
  let fs = state.borrow::<FileSystemRc>();
  let stat = fs.lstat_sync(&path).context_path("lstat", &path)?;
  let serializable_stat = SerializableStat::from(stat);
  serializable_stat.write(stat_out_buf);
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_fs_lstat_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
) -> Result<SerializableStat, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);
  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&path, "Deno.lstat()")?;
    state.borrow::<FileSystemRc>().clone()
  };
  let stat = fs
    .lstat_async(path.clone())
    .await
    .context_path("lstat", &path)?;
  Ok(SerializableStat::from(stat))
}

#[op2]
#[string]
pub fn op_fs_realpath_sync<P>(
  state: &mut OpState,
  #[string] path: String,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = state.borrow::<FileSystemRc>().clone();
  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&path, "Deno.realPathSync()")?;
  if path.is_relative() {
    permissions.check_read_blind(&fs.cwd()?, "CWD", "Deno.realPathSync()")?;
  }

  let resolved_path =
    fs.realpath_sync(&path).context_path("realpath", &path)?;

  let path_string = path_into_string(resolved_path.into_os_string())?;
  Ok(path_string)
}

#[op2(async)]
#[string]
pub async fn op_fs_realpath_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs;
  {
    let mut state = state.borrow_mut();
    fs = state.borrow::<FileSystemRc>().clone();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&path, "Deno.realPath()")?;
    if path.is_relative() {
      permissions.check_read_blind(&fs.cwd()?, "CWD", "Deno.realPath()")?;
    }
  }
  let resolved_path = fs
    .realpath_async(path.clone())
    .await
    .context_path("realpath", &path)?;

  let path_string = path_into_string(resolved_path.into_os_string())?;
  Ok(path_string)
}

#[op2]
#[serde]
pub fn op_fs_read_dir_sync<P>(
  state: &mut OpState,
  #[string] path: String,
) -> Result<Vec<FsDirEntry>, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  state
    .borrow_mut::<P>()
    .check_read(&path, "Deno.readDirSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  let entries = fs.read_dir_sync(&path).context_path("readdir", &path)?;

  Ok(entries)
}

#[op2(async)]
#[serde]
pub async fn op_fs_read_dir_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
) -> Result<Vec<FsDirEntry>, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = {
    let mut state = state.borrow_mut();
    state
      .borrow_mut::<P>()
      .check_read(&path, "Deno.readDir()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  let entries = fs
    .read_dir_async(path.clone())
    .await
    .context_path("readdir", &path)?;

  Ok(entries)
}

#[op2(fast)]
pub fn op_fs_rename_sync<P>(
  state: &mut OpState,
  #[string] oldpath: String,
  #[string] newpath: String,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(oldpath);
  let newpath = PathBuf::from(newpath);

  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&oldpath, "Deno.renameSync()")?;
  permissions.check_write(&oldpath, "Deno.renameSync()")?;
  permissions.check_write(&newpath, "Deno.renameSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.rename_sync(&oldpath, &newpath)
    .context_two_path("rename", &oldpath, &newpath)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_rename_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] oldpath: String,
  #[string] newpath: String,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(oldpath);
  let newpath = PathBuf::from(newpath);

  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&oldpath, "Deno.rename()")?;
    permissions.check_write(&oldpath, "Deno.rename()")?;
    permissions.check_write(&newpath, "Deno.rename()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.rename_async(oldpath.clone(), newpath.clone())
    .await
    .context_two_path("rename", &oldpath, &newpath)?;

  Ok(())
}

#[op2(fast)]
pub fn op_fs_link_sync<P>(
  state: &mut OpState,
  #[string] oldpath: &str,
  #[string] newpath: &str,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(oldpath);
  let newpath = PathBuf::from(newpath);

  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&oldpath, "Deno.linkSync()")?;
  permissions.check_write(&oldpath, "Deno.linkSync()")?;
  permissions.check_read(&newpath, "Deno.linkSync()")?;
  permissions.check_write(&newpath, "Deno.linkSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.link_sync(&oldpath, &newpath)
    .context_two_path("link", &oldpath, &newpath)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_link_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] oldpath: String,
  #[string] newpath: String,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(&oldpath);
  let newpath = PathBuf::from(&newpath);

  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&oldpath, "Deno.link()")?;
    permissions.check_write(&oldpath, "Deno.link()")?;
    permissions.check_read(&newpath, "Deno.link()")?;
    permissions.check_write(&newpath, "Deno.link()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.link_async(oldpath.clone(), newpath.clone())
    .await
    .context_two_path("link", &oldpath, &newpath)?;

  Ok(())
}

#[op2]
pub fn op_fs_symlink_sync<P>(
  state: &mut OpState,
  #[string] oldpath: &str,
  #[string] newpath: &str,
  #[serde] file_type: Option<FsFileType>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(oldpath);
  let newpath = PathBuf::from(newpath);

  let permissions = state.borrow_mut::<P>();
  permissions.check_write_all("Deno.symlinkSync()")?;
  permissions.check_read_all("Deno.symlinkSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.symlink_sync(&oldpath, &newpath, file_type)
    .context_two_path("symlink", &oldpath, &newpath)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_symlink_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] oldpath: String,
  #[string] newpath: String,
  #[serde] file_type: Option<FsFileType>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let oldpath = PathBuf::from(&oldpath);
  let newpath = PathBuf::from(&newpath);

  let fs = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_write_all("Deno.symlink()")?;
    permissions.check_read_all("Deno.symlink()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.symlink_async(oldpath.clone(), newpath.clone(), file_type)
    .await
    .context_two_path("symlink", &oldpath, &newpath)?;

  Ok(())
}

#[op2]
#[string]
pub fn op_fs_read_link_sync<P>(
  state: &mut OpState,
  #[string] path: String,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  state
    .borrow_mut::<P>()
    .check_read(&path, "Deno.readLink()")?;

  let fs = state.borrow::<FileSystemRc>();

  let target = fs.read_link_sync(&path).context_path("readlink", &path)?;
  let target_string = path_into_string(target.into_os_string())?;
  Ok(target_string)
}

#[op2(async)]
#[string]
pub async fn op_fs_read_link_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = {
    let mut state = state.borrow_mut();
    state
      .borrow_mut::<P>()
      .check_read(&path, "Deno.readLink()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  let target = fs
    .read_link_async(path.clone())
    .await
    .context_path("readlink", &path)?;
  let target_string = path_into_string(target.into_os_string())?;
  Ok(target_string)
}

#[op2(fast)]
pub fn op_fs_truncate_sync<P>(
  state: &mut OpState,
  #[string] path: &str,
  #[number] len: u64,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  state
    .borrow_mut::<P>()
    .check_write(&path, "Deno.truncateSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.truncate_sync(&path, len)
    .context_path("truncate", &path)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_truncate_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[number] len: u64,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = {
    let mut state = state.borrow_mut();
    state
      .borrow_mut::<P>()
      .check_write(&path, "Deno.truncate()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.truncate_async(path.clone(), len)
    .await
    .context_path("truncate", &path)?;

  Ok(())
}

#[op2(fast)]
pub fn op_fs_utime_sync<P>(
  state: &mut OpState,
  #[string] path: &str,
  #[number] atime_secs: i64,
  #[smi] atime_nanos: u32,
  #[number] mtime_secs: i64,
  #[smi] mtime_nanos: u32,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  state.borrow_mut::<P>().check_write(&path, "Deno.utime()")?;

  let fs = state.borrow::<FileSystemRc>();
  fs.utime_sync(&path, atime_secs, atime_nanos, mtime_secs, mtime_nanos)
    .context_path("utime", &path)?;

  Ok(())
}

#[op2(async)]
pub async fn op_fs_utime_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[number] atime_secs: i64,
  #[smi] atime_nanos: u32,
  #[number] mtime_secs: i64,
  #[smi] mtime_nanos: u32,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let fs = {
    let mut state = state.borrow_mut();
    state.borrow_mut::<P>().check_write(&path, "Deno.utime()")?;
    state.borrow::<FileSystemRc>().clone()
  };

  fs.utime_async(
    path.clone(),
    atime_secs,
    atime_nanos,
    mtime_secs,
    mtime_nanos,
  )
  .await
  .context_path("utime", &path)?;

  Ok(())
}

#[op2]
#[string]
pub fn op_fs_make_temp_dir_sync<P>(
  state: &mut OpState,
  #[string] dir: Option<String>,
  #[string] prefix: Option<String>,
  #[string] suffix: Option<String>,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let (dir, fs) = make_temp_check_sync::<P>(state, dir)?;

  let mut rng = thread_rng();

  const MAX_TRIES: u32 = 10;
  for _ in 0..MAX_TRIES {
    let path = tmp_name(&mut rng, &dir, prefix.as_deref(), suffix.as_deref())?;
    match fs.mkdir_sync(&path, false, 0o700) {
      Ok(_) => return path_into_string(path.into_os_string()),
      Err(FsError::Io(ref e)) if e.kind() == io::ErrorKind::AlreadyExists => {
        continue;
      }
      Err(e) => return Err(e).context("tmpdir"),
    }
  }

  Err(FsError::Io(io::Error::new(
    io::ErrorKind::AlreadyExists,
    "too many temp dirs exist",
  )))
  .context("tmpdir")
}

#[op2(async)]
#[string]
pub async fn op_fs_make_temp_dir_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] dir: Option<String>,
  #[string] prefix: Option<String>,
  #[string] suffix: Option<String>,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let (dir, fs) = make_temp_check_async::<P>(state, dir)?;

  let mut rng = thread_rng();

  const MAX_TRIES: u32 = 10;
  for _ in 0..MAX_TRIES {
    let path = tmp_name(&mut rng, &dir, prefix.as_deref(), suffix.as_deref())?;
    match fs.clone().mkdir_async(path.clone(), false, 0o700).await {
      Ok(_) => return path_into_string(path.into_os_string()),
      Err(FsError::Io(ref e)) if e.kind() == io::ErrorKind::AlreadyExists => {
        continue;
      }
      Err(e) => return Err(e).context("tmpdir"),
    }
  }

  Err(FsError::Io(io::Error::new(
    io::ErrorKind::AlreadyExists,
    "too many temp dirs exist",
  )))
  .context("tmpdir")
}

#[op2]
#[string]
pub fn op_fs_make_temp_file_sync<P>(
  state: &mut OpState,
  #[string] dir: Option<String>,
  #[string] prefix: Option<String>,
  #[string] suffix: Option<String>,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let (dir, fs) = make_temp_check_sync::<P>(state, dir)?;

  let open_opts = OpenOptions {
    write: true,
    create_new: true,
    mode: Some(0o600),
    ..Default::default()
  };

  let mut rng = thread_rng();

  const MAX_TRIES: u32 = 10;
  for _ in 0..MAX_TRIES {
    let path = tmp_name(&mut rng, &dir, prefix.as_deref(), suffix.as_deref())?;
    match fs.open_sync(&path, open_opts) {
      Ok(_) => return path_into_string(path.into_os_string()),
      Err(FsError::Io(ref e)) if e.kind() == io::ErrorKind::AlreadyExists => {
        continue;
      }
      Err(e) => return Err(e).context("tmpfile"),
    }
  }

  Err(FsError::Io(io::Error::new(
    io::ErrorKind::AlreadyExists,
    "too many temp files exist",
  )))
  .context("tmpfile")
}

#[op2(async)]
#[string]
pub async fn op_fs_make_temp_file_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] dir: Option<String>,
  #[string] prefix: Option<String>,
  #[string] suffix: Option<String>,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let (dir, fs) = make_temp_check_async::<P>(state, dir)?;

  let open_opts = OpenOptions {
    write: true,
    create_new: true,
    mode: Some(0o600),
    ..Default::default()
  };

  let mut rng = thread_rng();

  const MAX_TRIES: u32 = 10;
  for _ in 0..MAX_TRIES {
    let path = tmp_name(&mut rng, &dir, prefix.as_deref(), suffix.as_deref())?;
    match fs.clone().open_async(path.clone(), open_opts).await {
      Ok(_) => return path_into_string(path.into_os_string()),
      Err(FsError::Io(ref e)) if e.kind() == io::ErrorKind::AlreadyExists => {
        continue;
      }
      Err(e) => return Err(e).context("tmpfile"),
    }
  }
  Err(FsError::Io(io::Error::new(
    io::ErrorKind::AlreadyExists,
    "too many temp files exist",
  )))
  .context("tmpfile")
}

fn make_temp_check_sync<P>(
  state: &mut OpState,
  dir: Option<String>,
) -> Result<(PathBuf, FileSystemRc), AnyError>
where
  P: FsPermissions + 'static,
{
  let fs = state.borrow::<FileSystemRc>().clone();
  let dir = match dir {
    Some(dir) => {
      let dir = PathBuf::from(dir);
      state
        .borrow_mut::<P>()
        .check_write(&dir, "Deno.makeTempFile()")?;
      dir
    }
    None => {
      let dir = fs.tmp_dir().context("tmpdir")?;
      state.borrow_mut::<P>().check_write_blind(
        &dir,
        "TMP",
        "Deno.makeTempFile()",
      )?;
      dir
    }
  };
  Ok((dir, fs))
}

fn make_temp_check_async<P>(
  state: Rc<RefCell<OpState>>,
  dir: Option<String>,
) -> Result<(PathBuf, FileSystemRc), AnyError>
where
  P: FsPermissions + 'static,
{
  let mut state = state.borrow_mut();
  let fs = state.borrow::<FileSystemRc>().clone();
  let dir = match dir {
    Some(dir) => {
      let dir = PathBuf::from(dir);
      state
        .borrow_mut::<P>()
        .check_write(&dir, "Deno.makeTempFile()")?;
      dir
    }
    None => {
      let dir = fs.tmp_dir().context("tmpdir")?;
      state.borrow_mut::<P>().check_write_blind(
        &dir,
        "TMP",
        "Deno.makeTempFile()",
      )?;
      dir
    }
  };
  Ok((dir, fs))
}

fn tmp_name(
  rng: &mut ThreadRng,
  dir: &Path,
  prefix: Option<&str>,
  suffix: Option<&str>,
) -> Result<PathBuf, AnyError> {
  let prefix = prefix.unwrap_or("");
  let suffix = suffix.unwrap_or("");

  let mut path = dir.join("_");

  let unique = rng.gen::<u32>();
  path.set_file_name(format!("{prefix}{unique:08x}{suffix}"));

  Ok(path)
}

#[op2]
pub fn op_fs_write_file_sync<P>(
  state: &mut OpState,
  #[string] path: String,
  mode: Option<u32>,
  append: bool,
  create: bool,
  create_new: bool,
  #[buffer] data: JsBuffer,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let permissions = state.borrow_mut::<P>();
  let options = OpenOptions::write(create, append, create_new, mode);
  permissions.check(&options, &path, "Deno.writeFileSync()")?;

  let fs = state.borrow::<FileSystemRc>();

  fs.write_file_sync(&path, options, &data)
    .context_path("writefile", &path)?;

  Ok(())
}

#[op2(async)]
#[allow(clippy::too_many_arguments)]
pub async fn op_fs_write_file_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[smi] mode: Option<u32>,
  append: bool,
  create: bool,
  create_new: bool,
  #[buffer] data: JsBuffer,
  #[smi] cancel_rid: Option<ResourceId>,
) -> Result<(), AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let options = OpenOptions::write(create, append, create_new, mode);

  let (fs, cancel_handle) = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check(&options, &path, "Deno.writeFile()")?;
    let cancel_handle = cancel_rid
      .and_then(|rid| state.resource_table.get::<CancelHandle>(rid).ok());
    (state.borrow::<FileSystemRc>().clone(), cancel_handle)
  };

  let fut = fs.write_file_async(path.clone(), options, data.to_vec());

  if let Some(cancel_handle) = cancel_handle {
    let res = fut.or_cancel(cancel_handle).await;

    if let Some(cancel_rid) = cancel_rid {
      if let Ok(res) = state.borrow_mut().resource_table.take_any(cancel_rid) {
        res.close();
      }
    };

    res?.context_path("writefile", &path)?;
  } else {
    fut.await.context_path("writefile", &path)?;
  }

  Ok(())
}

#[op2]
#[serde]
pub fn op_fs_read_file_sync<P>(
  state: &mut OpState,
  #[string] path: String,
) -> Result<ToJsBuffer, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&path, "Deno.readFileSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  let buf = fs.read_file_sync(&path).context_path("readfile", &path)?;

  Ok(buf.into())
}

#[op2(async)]
#[serde]
pub async fn op_fs_read_file_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[smi] cancel_rid: Option<ResourceId>,
) -> Result<ToJsBuffer, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let (fs, cancel_handle) = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&path, "Deno.readFile()")?;
    let cancel_handle = cancel_rid
      .and_then(|rid| state.resource_table.get::<CancelHandle>(rid).ok());
    (state.borrow::<FileSystemRc>().clone(), cancel_handle)
  };

  let fut = fs.read_file_async(path.clone());

  let buf = if let Some(cancel_handle) = cancel_handle {
    let res = fut.or_cancel(cancel_handle).await;

    if let Some(cancel_rid) = cancel_rid {
      if let Ok(res) = state.borrow_mut().resource_table.take_any(cancel_rid) {
        res.close();
      }
    };

    res?.context_path("readfile", &path)?
  } else {
    fut.await.context_path("readfile", &path)?
  };

  Ok(buf.into())
}

#[op2]
#[string]
pub fn op_fs_read_file_text_sync<P>(
  state: &mut OpState,
  #[string] path: String,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let permissions = state.borrow_mut::<P>();
  permissions.check_read(&path, "Deno.readFileSync()")?;

  let fs = state.borrow::<FileSystemRc>();
  let buf = fs.read_file_sync(&path).context_path("readfile", &path)?;

  Ok(string_from_utf8_lossy(buf))
}

#[op2(async)]
#[string]
pub async fn op_fs_read_file_text_async<P>(
  state: Rc<RefCell<OpState>>,
  #[string] path: String,
  #[smi] cancel_rid: Option<ResourceId>,
) -> Result<String, AnyError>
where
  P: FsPermissions + 'static,
{
  let path = PathBuf::from(path);

  let (fs, cancel_handle) = {
    let mut state = state.borrow_mut();
    let permissions = state.borrow_mut::<P>();
    permissions.check_read(&path, "Deno.readFile()")?;
    let cancel_handle = cancel_rid
      .and_then(|rid| state.resource_table.get::<CancelHandle>(rid).ok());
    (state.borrow::<FileSystemRc>().clone(), cancel_handle)
  };

  let fut = fs.read_file_async(path.clone());

  let buf = if let Some(cancel_handle) = cancel_handle {
    let res = fut.or_cancel(cancel_handle).await;

    if let Some(cancel_rid) = cancel_rid {
      if let Ok(res) = state.borrow_mut().resource_table.take_any(cancel_rid) {
        res.close();
      }
    };

    res?.context_path("readfile", &path)?
  } else {
    fut.await.context_path("readfile", &path)?
  };

  Ok(string_from_utf8_lossy(buf))
}

// Like String::from_utf8_lossy but operates on owned values
fn string_from_utf8_lossy(buf: Vec<u8>) -> String {
  match String::from_utf8_lossy(&buf) {
    // buf contained non-utf8 chars than have been patched
    Cow::Owned(s) => s,
    // SAFETY: if Borrowed then the buf only contains utf8 chars,
    // we do this instead of .into_owned() to avoid copying the input buf
    Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(buf) },
  }
}

fn to_seek_from(offset: i64, whence: i32) -> Result<SeekFrom, AnyError> {
  let seek_from = match whence {
    0 => SeekFrom::Start(offset as u64),
    1 => SeekFrom::Current(offset),
    2 => SeekFrom::End(offset),
    _ => {
      return Err(type_error(format!("Invalid seek mode: {whence}")));
    }
  };
  Ok(seek_from)
}

#[op2(fast)]
#[number]
pub fn op_fs_seek_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
  #[number] offset: i64,
  #[smi] whence: i32,
) -> Result<u64, AnyError> {
  let pos = to_seek_from(offset, whence)?;
  let file = FileResource::get_file(state, rid)?;
  let cursor = file.seek_sync(pos)?;
  Ok(cursor)
}

#[op2(async)]
#[number]
pub async fn op_fs_seek_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
  #[number] offset: i64,
  #[smi] whence: i32,
) -> Result<u64, AnyError> {
  let pos = to_seek_from(offset, whence)?;
  let file = FileResource::get_file(&state.borrow(), rid)?;
  let cursor = file.seek_async(pos).await?;
  Ok(cursor)
}

#[op2(fast)]
pub fn op_fs_fdatasync_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(state, rid)?;
  file.datasync_sync()?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_fdatasync_sync_unstable(
  state: &mut OpState,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(state, "Deno.FsFile.syncDataSync");
  let file = FileResource::get_file(state, rid)?;
  file.datasync_sync()?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_fdatasync_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.datasync_async().await?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_fdatasync_async_unstable(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(&state.borrow(), "Deno.FsFile.syncData");
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.datasync_async().await?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_fsync_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(state, rid)?;
  file.sync_sync()?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_fsync_sync_unstable(
  state: &mut OpState,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(state, "Deno.FsFile.syncSync");
  let file = FileResource::get_file(state, rid)?;
  file.sync_sync()?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_fsync_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.sync_async().await?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_fsync_async_unstable(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(&state.borrow(), "Deno.FsFile.sync");
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.sync_async().await?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_fstat_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
  #[buffer] stat_out_buf: &mut [u32],
) -> Result<(), AnyError> {
  let file = FileResource::get_file(state, rid)?;
  let stat = file.stat_sync()?;
  let serializable_stat = SerializableStat::from(stat);
  serializable_stat.write(stat_out_buf);
  Ok(())
}

#[op2(async)]
#[serde]
pub async fn op_fs_fstat_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<SerializableStat, AnyError> {
  let file = FileResource::get_file(&state.borrow(), rid)?;
  let stat = file.stat_async().await?;
  Ok(stat.into())
}

#[op2(fast)]
pub fn op_fs_flock_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
  exclusive: bool,
) -> Result<(), AnyError> {
  check_unstable(state, "Deno.flockSync");
  let file = FileResource::get_file(state, rid)?;
  file.lock_sync(exclusive)?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_flock_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
  exclusive: bool,
) -> Result<(), AnyError> {
  check_unstable(&state.borrow(), "Deno.flock");
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.lock_async(exclusive).await?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_funlock_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(state, "Deno.funlockSync");
  let file = FileResource::get_file(state, rid)?;
  file.unlock_sync()?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_funlock_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
) -> Result<(), AnyError> {
  check_unstable(&state.borrow(), "Deno.funlock");
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.unlock_async().await?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_ftruncate_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
  #[number] len: u64,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(state, rid)?;
  file.truncate_sync(len)?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_ftruncate_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
  #[number] len: u64,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file.truncate_async(len).await?;
  Ok(())
}

#[op2(fast)]
pub fn op_fs_futime_sync(
  state: &mut OpState,
  #[smi] rid: ResourceId,
  #[number] atime_secs: i64,
  #[smi] atime_nanos: u32,
  #[number] mtime_secs: i64,
  #[smi] mtime_nanos: u32,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(state, rid)?;
  file.utime_sync(atime_secs, atime_nanos, mtime_secs, mtime_nanos)?;
  Ok(())
}

#[op2(async)]
pub async fn op_fs_futime_async(
  state: Rc<RefCell<OpState>>,
  #[smi] rid: ResourceId,
  #[number] atime_secs: i64,
  #[smi] atime_nanos: u32,
  #[number] mtime_secs: i64,
  #[smi] mtime_nanos: u32,
) -> Result<(), AnyError> {
  let file = FileResource::get_file(&state.borrow(), rid)?;
  file
    .utime_async(atime_secs, atime_nanos, mtime_secs, mtime_nanos)
    .await?;
  Ok(())
}

trait WithContext {
  fn context<E: Into<Box<dyn std::error::Error + Send + Sync>>>(
    self,
    desc: E,
  ) -> AnyError;
}

impl WithContext for FsError {
  fn context<E: Into<Box<dyn std::error::Error + Send + Sync>>>(
    self,
    desc: E,
  ) -> AnyError {
    match self {
      FsError::Io(io) => {
        AnyError::new(io::Error::new(io.kind(), desc)).context(io)
      }
      _ => self.into(),
    }
  }
}

trait MapErrContext {
  type R;

  fn context_fn<F, E>(self, f: F) -> Self::R
  where
    F: FnOnce() -> E,
    E: Into<Box<dyn std::error::Error + Send + Sync>>;

  fn context(self, desc: &'static str) -> Self::R;

  fn context_path(self, operation: &'static str, path: &Path) -> Self::R;

  fn context_two_path(
    self,
    operation: &'static str,
    from: &Path,
    to: &Path,
  ) -> Self::R;
}

impl<T> MapErrContext for Result<T, FsError> {
  type R = Result<T, AnyError>;

  fn context_fn<F, E>(self, f: F) -> Self::R
  where
    F: FnOnce() -> E,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
  {
    self.map_err(|err| {
      let message = f();
      err.context(message)
    })
  }

  fn context(self, desc: &'static str) -> Self::R {
    self.context_fn(move || desc)
  }

  fn context_path(self, operation: &'static str, path: &Path) -> Self::R {
    self.context_fn(|| format!("{operation} '{}'", path.display()))
  }

  fn context_two_path(
    self,
    operation: &'static str,
    oldpath: &Path,
    newpath: &Path,
  ) -> Self::R {
    self.context_fn(|| {
      format!(
        "{operation} '{}' -> '{}'",
        oldpath.display(),
        newpath.display()
      )
    })
  }
}

fn path_into_string(s: std::ffi::OsString) -> Result<String, AnyError> {
  s.into_string().map_err(|s| {
    let message = format!("File name or path {s:?} is not valid UTF-8");
    custom_error("InvalidData", message)
  })
}

macro_rules! create_struct_writer {
  (pub struct $name:ident { $($field:ident: $type:ty),* $(,)? }) => {
    impl $name {
      fn write(self, buf: &mut [u32]) {
        let mut offset = 0;
        $(
          let value = self.$field as u64;
          buf[offset] = value as u32;
          buf[offset + 1] = (value >> 32) as u32;
          #[allow(unused_assignments)]
          {
            offset += 2;
          }
        )*
      }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct $name {
      $($field: $type),*
    }
  };
}

create_struct_writer! {
  pub struct SerializableStat {
    is_file: bool,
    is_directory: bool,
    is_symlink: bool,
    size: u64,
    // In milliseconds, like JavaScript. Available on both Unix or Windows.
    mtime_set: bool,
    mtime: u64,
    atime_set: bool,
    atime: u64,
    birthtime_set: bool,
    birthtime: u64,
    // Following are only valid under Unix.
    dev: u64,
    ino: u64,
    mode: u32,
    nlink: u64,
    uid: u32,
    gid: u32,
    rdev: u64,
    blksize: u64,
    blocks: u64,
    is_block_device: bool,
    is_char_device: bool,
    is_fifo: bool,
    is_socket: bool,
  }
}

impl From<FsStat> for SerializableStat {
  fn from(stat: FsStat) -> Self {
    SerializableStat {
      is_file: stat.is_file,
      is_directory: stat.is_directory,
      is_symlink: stat.is_symlink,
      size: stat.size,

      mtime_set: stat.mtime.is_some(),
      mtime: stat.mtime.unwrap_or(0),
      atime_set: stat.atime.is_some(),
      atime: stat.atime.unwrap_or(0),
      birthtime_set: stat.birthtime.is_some(),
      birthtime: stat.birthtime.unwrap_or(0),

      dev: stat.dev,
      ino: stat.ino,
      mode: stat.mode,
      nlink: stat.nlink,
      uid: stat.uid,
      gid: stat.gid,
      rdev: stat.rdev,
      blksize: stat.blksize,
      blocks: stat.blocks,
      is_block_device: stat.is_block_device,
      is_char_device: stat.is_char_device,
      is_fifo: stat.is_fifo,
      is_socket: stat.is_socket,
    }
  }
}
