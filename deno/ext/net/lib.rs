// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

pub mod io;
pub mod ops;
pub mod ops_tls;
#[cfg(unix)]
pub mod ops_unix;
pub mod raw;
pub mod resolve_addr;

use deno_core::error::AnyError;
use deno_core::op2;
use deno_core::OpState;
use deno_tls::rustls::RootCertStore;
use deno_tls::RootCertStoreProvider;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

pub const UNSTABLE_FEATURE_NAME: &str = "net";

pub trait NetPermissions {
  fn check_net<T: AsRef<str>>(
    &mut self,
    _host: &(T, Option<u16>),
    _api_name: &str,
  ) -> Result<(), AnyError>;
  fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<(), AnyError>;
  fn check_write(&mut self, _p: &Path, _api_name: &str)
    -> Result<(), AnyError>;
}

/// Helper for checking unstable features. Used for sync ops.
fn check_unstable(state: &OpState, api_name: &str) {
  // TODO(bartlomieju): replace with `state.feature_checker.check_or_exit`
  // once we phase out `check_or_exit_with_legacy_fallback`
  state
    .feature_checker
    .check_or_exit_with_legacy_fallback(UNSTABLE_FEATURE_NAME, api_name);
}

pub fn get_declaration() -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib.deno_net.d.ts")
}

#[derive(Clone)]
pub struct DefaultTlsOptions {
  pub root_cert_store_provider: Option<Arc<dyn RootCertStoreProvider>>,
}

impl DefaultTlsOptions {
  pub fn root_cert_store(&self) -> Result<Option<RootCertStore>, AnyError> {
    Ok(match &self.root_cert_store_provider {
      Some(provider) => Some(provider.get_or_try_init()?.clone()),
      None => None,
    })
  }
}

/// `UnsafelyIgnoreCertificateErrors` is a wrapper struct so it can be placed inside `GothamState`;
/// using type alias for a `Option<Vec<String>>` could work, but there's a high chance
/// that there might be another type alias pointing to a `Option<Vec<String>>`, which
/// would override previously used alias.
pub struct UnsafelyIgnoreCertificateErrors(pub Option<Vec<String>>);

deno_core::extension!(deno_net,
  deps = [ deno_web ],
  parameters = [ P: NetPermissions ],
  ops = [
    ops::op_net_accept_tcp,
    ops::op_net_connect_tcp<P>,
    ops::op_net_listen_tcp<P>,
    ops::op_net_listen_udp<P>,
    ops::op_node_unstable_net_listen_udp<P>,
    ops::op_net_recv_udp,
    ops::op_net_send_udp<P>,
    ops::op_net_join_multi_v4_udp,
    ops::op_net_join_multi_v6_udp,
    ops::op_net_leave_multi_v4_udp,
    ops::op_net_leave_multi_v6_udp,
    ops::op_net_set_multi_loopback_udp,
    ops::op_net_set_multi_ttl_udp,
    ops::op_dns_resolve<P>,
    ops::op_set_nodelay,
    ops::op_set_keepalive,

    ops_tls::op_tls_start<P>,
    ops_tls::op_net_connect_tls<P>,
    ops_tls::op_net_listen_tls<P>,
    ops_tls::op_net_accept_tls,
    ops_tls::op_tls_handshake,

    #[cfg(unix)] ops_unix::op_net_accept_unix,
    #[cfg(unix)] ops_unix::op_net_connect_unix<P>,
    #[cfg(unix)] ops_unix::op_net_listen_unix<P>,
    #[cfg(unix)] ops_unix::op_net_listen_unixpacket<P>,
    #[cfg(unix)] ops_unix::op_node_unstable_net_listen_unixpacket<P>,
    #[cfg(unix)] ops_unix::op_net_recv_unixpacket,
    #[cfg(unix)] ops_unix::op_net_send_unixpacket<P>,

    #[cfg(not(unix))] op_net_accept_unix,
    #[cfg(not(unix))] op_net_connect_unix,
    #[cfg(not(unix))] op_net_listen_unix,
    #[cfg(not(unix))] op_net_listen_unixpacket,
    #[cfg(not(unix))] op_node_unstable_net_listen_unixpacket,
    #[cfg(not(unix))] op_net_recv_unixpacket,
    #[cfg(not(unix))] op_net_send_unixpacket,
  ],
  esm = [ "01_net.js", "02_tls.js" ],
  options = {
    root_cert_store_provider: Option<Arc<dyn RootCertStoreProvider>>,
    unsafely_ignore_certificate_errors: Option<Vec<String>>,
  },
  state = |state, options| {
    state.put(DefaultTlsOptions {
      root_cert_store_provider: options.root_cert_store_provider,
    });
    state.put(UnsafelyIgnoreCertificateErrors(
      options.unsafely_ignore_certificate_errors,
    ));
  },
);

macro_rules! stub_op {
  ($name:ident) => {
    #[op2(fast)]
    fn $name() {
      panic!("Unsupported on non-unix platforms")
    }
  };
}

stub_op!(op_net_accept_unix);
stub_op!(op_net_connect_unix);
stub_op!(op_net_listen_unix);
stub_op!(op_net_listen_unixpacket);
stub_op!(op_node_unstable_net_listen_unixpacket);
stub_op!(op_net_recv_unixpacket);
stub_op!(op_net_send_unixpacket);
