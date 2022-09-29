
use crate::certificate_authority::{CACHE_TTL};

use http::uri::Authority;
use moka::future::Cache;
use openssl::{
    pkey::{PKey, Private},
    hash::MessageDigest,
    x509::{X509},
};
use std::{
    sync::Arc,
    time::{Duration}
};
use tokio_rustls::rustls::{self, ServerConfig};


#[cfg_attr(docsrs, doc(cfg(feature = "openssl-ca")))]
#[derive(Clone)]
pub struct OpensslAuthority {
    pkey: PKey<Private>,
    private_key: rustls::PrivateKey,
    ca_cert: X509,
    hash: MessageDigest,
    cache: Cache<Authority, Arc<ServerConfig>>
}

impl OpensslAuthority {
    pub fn new(pkey: PKey<Private>, ca_cert: X509, hash: MessageDigest, cache_size: u64) -> Self {
        let private_key = rustls::PrivateKey(
            pkey.private_key_to_der()
                .expect("Failed to encode private key"),
        );

        Self {
            pkey,
            private_key,
            ca_cert,
            hash, 
            cache: Cache::builder()
                .max_capacity(cache_size)
                .time_to_live(Duration::from_secs(CACHE_TTL))
                .build(),
        }
    }
}