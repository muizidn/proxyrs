use async_trait::async_trait;
use http::uri::Authority;
use tokio_rustls::rustls::ServerConfig;
use std::{
    sync::Arc,
};

#[cfg(feature = "openssl-ca")]
pub mod openssl_authority;

#[cfg(feature = "openssl-ca")]
pub use openssl_authority::*;

const TTL_SECS: i64 = 365 * 24 * 60 * 60;
const CACHE_TTL: u64 = TTL_SECS as u64 / 2;
const NOT_BEFORE_OFFSET: i64 = 60;

#[async_trait]
pub trait CertificateAuthority: Send + Sync + 'static {
    async fn gen_server_config(&self, authority: &Authority) -> Arc<ServerConfig>;
}