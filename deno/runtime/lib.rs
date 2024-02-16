// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

pub use deno_broadcast_channel;
pub use deno_cache;
pub use deno_canvas;
pub use deno_console;
pub use deno_core;
pub use deno_cron;
pub use deno_crypto;
pub use deno_fetch;
pub use deno_ffi;
pub use deno_fs;
pub use deno_http;
pub use deno_io;
pub use deno_kv;
pub use deno_napi;
pub use deno_net;
pub use deno_node;
pub use deno_tls;
pub use deno_url;
pub use deno_web;
pub use deno_webgpu;
pub use deno_webidl;
pub use deno_websocket;
pub use deno_webstorage;

pub mod colors;
pub mod errors;
pub mod fmt_errors;
pub mod fs_util;
pub mod inspector_server;
pub mod js;
pub mod ops;
pub mod permissions;
pub mod snapshot;
pub mod tokio_util;
pub mod web_worker;
pub mod worker;

mod worker_bootstrap;
pub use worker_bootstrap::BootstrapOptions;
pub use worker_bootstrap::WorkerLogLevel;

mod shared;
pub use shared::runtime;

// NOTE(bartlomieju): keep IDs in sync with `runtime/90_deno_ns.js` (search for `unstableFeatures`)
pub static UNSTABLE_GRANULAR_FLAGS: &[(
  // flag name
  &str,
  // help text
  &str,
  // id to enable it in runtime/99_main.js
  i32,
)] = &[
  (
    deno_broadcast_channel::UNSTABLE_FEATURE_NAME,
    "Enable unstable `BroadcastChannel` API",
    1,
  ),
  (
    deno_cron::UNSTABLE_FEATURE_NAME,
    "Enable unstable Deno.cron API",
    2,
  ),
  (
    deno_ffi::UNSTABLE_FEATURE_NAME,
    "Enable unstable FFI APIs",
    3,
  ),
  (
    deno_fs::UNSTABLE_FEATURE_NAME,
    "Enable unstable file system APIs",
    4,
  ),
  (
    ops::http::UNSTABLE_FEATURE_NAME,
    "Enable unstable HTTP APIs",
    5,
  ),
  (
    deno_kv::UNSTABLE_FEATURE_NAME,
    "Enable unstable Key-Value store APIs",
    6,
  ),
  (
    deno_net::UNSTABLE_FEATURE_NAME,
    "Enable unstable net APIs",
    7,
  ),
  (
    "temporal",
    "Enable unstable Temporal API",
    // Not used in JS
    8,
  ),
  (
    "unsafe-proto",
    "Enable unsafe __proto__ support. This is a security risk.",
    // This number is used directly in the JS code. Search
    // for "unstableIds" to see where it's used.
    9,
  ),
  (
    deno_webgpu::UNSTABLE_FEATURE_NAME,
    "Enable unstable `WebGPU` API",
    10,
  ),
  (
    ops::worker_host::UNSTABLE_FEATURE_NAME,
    "Enable unstable Web Worker APIs",
    11,
  ),
];

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn unstable_granular_flag_names_sorted() {
    let flags = UNSTABLE_GRANULAR_FLAGS
      .iter()
      .map(|(name, _, _)| name.to_string())
      .collect::<Vec<_>>();
    let mut sorted_flags = flags.clone();
    sorted_flags.sort();
    // sort the flags by name so they appear nicely in the help text
    assert_eq!(flags, sorted_flags);
  }
}
