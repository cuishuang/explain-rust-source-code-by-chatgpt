// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

mod interface;
mod ops;
mod std_fs;
pub mod sync;

pub use crate::interface::FileSystem;
pub use crate::interface::FileSystemRc;
pub use crate::interface::FsDirEntry;
pub use crate::interface::FsFileType;
pub use crate::interface::OpenOptions;
pub use crate::std_fs::RealFs;
pub use crate::sync::MaybeSend;
pub use crate::sync::MaybeSync;

use crate::ops::*;

use deno_core::error::AnyError;
use deno_core::OpState;
use std::path::Path;

pub trait FsPermissions {
  fn check_read(&mut self, path: &Path, api_name: &str)
    -> Result<(), AnyError>;
  fn check_read_all(&mut self, api_name: &str) -> Result<(), AnyError>;
  fn check_read_blind(
    &mut self,
    p: &Path,
    display: &str,
    api_name: &str,
  ) -> Result<(), AnyError>;
  fn check_write(
    &mut self,
    path: &Path,
    api_name: &str,
  ) -> Result<(), AnyError>;
  fn check_write_partial(
    &mut self,
    path: &Path,
    api_name: &str,
  ) -> Result<(), AnyError>;
  fn check_write_all(&mut self, api_name: &str) -> Result<(), AnyError>;
  fn check_write_blind(
    &mut self,
    p: &Path,
    display: &str,
    api_name: &str,
  ) -> Result<(), AnyError>;

  fn check(
    &mut self,
    open_options: &OpenOptions,
    path: &Path,
    api_name: &str,
  ) -> Result<(), AnyError> {
    if open_options.read {
      self.check_read(path, api_name)?;
    }
    if open_options.write || open_options.append {
      self.check_write(path, api_name)?;
    }
    Ok(())
  }
}

pub const UNSTABLE_FEATURE_NAME: &str = "fs";

/// Helper for checking unstable features. Used for sync ops.
fn check_unstable(state: &OpState, api_name: &str) {
  // TODO(bartlomieju): replace with `state.feature_checker.check_or_exit`
  // once we phase out `check_or_exit_with_legacy_fallback`
  state
    .feature_checker
    .check_or_exit_with_legacy_fallback(UNSTABLE_FEATURE_NAME, api_name);
}

deno_core::extension!(deno_fs,
  deps = [ deno_web ],
  parameters = [P: FsPermissions],
  ops = [
    op_fs_cwd<P>,
    op_fs_umask,
    op_fs_chdir<P>,

    op_fs_open_sync<P>,
    op_fs_open_async<P>,
    op_fs_mkdir_sync<P>,
    op_fs_mkdir_async<P>,
    op_fs_chmod_sync<P>,
    op_fs_chmod_async<P>,
    op_fs_chown_sync<P>,
    op_fs_chown_async<P>,
    op_fs_remove_sync<P>,
    op_fs_remove_async<P>,
    op_fs_copy_file_sync<P>,
    op_fs_copy_file_async<P>,
    op_fs_stat_sync<P>,
    op_fs_stat_async<P>,
    op_fs_lstat_sync<P>,
    op_fs_lstat_async<P>,
    op_fs_realpath_sync<P>,
    op_fs_realpath_async<P>,
    op_fs_read_dir_sync<P>,
    op_fs_read_dir_async<P>,
    op_fs_rename_sync<P>,
    op_fs_rename_async<P>,
    op_fs_link_sync<P>,
    op_fs_link_async<P>,
    op_fs_symlink_sync<P>,
    op_fs_symlink_async<P>,
    op_fs_read_link_sync<P>,
    op_fs_read_link_async<P>,
    op_fs_truncate_sync<P>,
    op_fs_truncate_async<P>,
    op_fs_utime_sync<P>,
    op_fs_utime_async<P>,
    op_fs_make_temp_dir_sync<P>,
    op_fs_make_temp_dir_async<P>,
    op_fs_make_temp_file_sync<P>,
    op_fs_make_temp_file_async<P>,
    op_fs_write_file_sync<P>,
    op_fs_write_file_async<P>,
    op_fs_read_file_sync<P>,
    op_fs_read_file_async<P>,
    op_fs_read_file_text_sync<P>,
    op_fs_read_file_text_async<P>,

    op_fs_seek_sync,
    op_fs_seek_async,
    op_fs_fdatasync_sync,
    op_fs_fdatasync_async,
    op_fs_fdatasync_sync_unstable,
    op_fs_fdatasync_async_unstable,
    op_fs_fsync_sync,
    op_fs_fsync_async,
    op_fs_fsync_sync_unstable,
    op_fs_fsync_async_unstable,
    op_fs_fstat_sync,
    op_fs_fstat_async,
    op_fs_flock_sync,
    op_fs_flock_async,
    op_fs_funlock_sync,
    op_fs_funlock_async,
    op_fs_ftruncate_sync,
    op_fs_ftruncate_async,
    op_fs_futime_sync,
    op_fs_futime_async,

  ],
  esm = [ "30_fs.js" ],
  options = {
    fs: FileSystemRc,
  },
  state = |state, options| {
    state.put(options.fs);
  },
);
