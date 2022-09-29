#[cfg(feature = "openssl-ca")]
pub mod openssl_authority;

#[cfg(feature = "openssl-ca")]
pub use openssl_authority::*;

const TTL_SECS: i64 = 365 * 24 * 60 * 60;
const CACHE_TTL: u64 = TTL_SECS as u64 / 2;