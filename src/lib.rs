pub mod certificate_authority;
mod proxy;
mod noop;
mod error;

pub use proxy::*;
use noop::*;

pub use async_trait;
#[cfg(feature = "openssl-ca")]
pub use openssl;
pub use hyper;
pub use tokio_tungstenite;

pub trait HttpHandler: Clone + Send + Sync + 'static {
}

pub trait WebSocketHandler: Clone + Send + Sync + 'static {
}