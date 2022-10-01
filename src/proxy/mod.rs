use std::{sync::{Arc}, future::Future};
use hyper::{
    Client, client::connect::Connect
};
use tokio_tungstenite::Connector;

pub mod builder;

use builder::{ProxyBuilder, AddrListenerServer, WantsAddr};

use crate::{
    certificate_authority::CertificateAuthority,
    error::Error,
    HttpHandler,
    WebSocketHandler,
};

pub struct Proxy<C, CA, H, W> {
    als: AddrListenerServer,
    ca: Arc<CA>,
    client: Client<C>,
    http_handler: H,
    websocket_handler: W,
    websocket_connector: Option<Connector>
}

impl Proxy<(), (), (), ()> {
    pub fn builder() -> ProxyBuilder<WantsAddr> {
        ProxyBuilder::new()
    }
}

impl <C, CA, H, W> Proxy<C, CA, H, W>
where 
    C: Connect + Clone + Send + Sync + 'static,
    CA: CertificateAuthority,
    H: HttpHandler,
    W: WebSocketHandler,
{
    pub async fn start<F: Future<Output = ()>>(self, shutdown_signal: F) -> Result<(), Error> {
        Err(Error::Unknown)
    }

}