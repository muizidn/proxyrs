
use crate::{HttpHandler, WebSocketHandler};
use crate::{
    certificate_authority::CertificateAuthority,
    NoopHandler, Proxy,
};
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;

use hyper::{server::conn::AddrIncoming, client::HttpConnector, Client};
#[cfg(feature = "rustls-client")]
use hyper_rustls::{HttpsConnector as RustlsConnector, HttpsConnectorBuilder};
use tokio_tungstenite::Connector;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProxyBuilder<T>(T);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WantsAddr(());

#[derive(Debug)]
pub(crate) enum AddrListenerServer {
    Addr(SocketAddr),
    Listener(TcpListener),
    Server(Box<hyper::server::Builder<AddrIncoming>>),
}

impl ProxyBuilder<WantsAddr> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_addr(self, addr: SocketAddr) -> ProxyBuilder<WantsClient> {
        ProxyBuilder(WantsClient{
            als: AddrListenerServer::Addr(addr),
        })
    }
}

impl Default for ProxyBuilder<WantsAddr> {
    fn default() -> Self {
        ProxyBuilder(WantsAddr(()))
    }
}

#[derive(Debug)]
pub struct WantsClient {
    als: AddrListenerServer,
}

impl ProxyBuilder<WantsClient> {
    pub fn with_rustls_client(self) -> ProxyBuilder<WantsCa<RustlsConnector<HttpConnector>>> {
        let https = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1();

        #[cfg(feature = "http2")]
        let https = https.enable_http2();
        
        let https = https.build();

        ProxyBuilder(WantsCa{
            als: self.0.als,
            client: Client::builder()
                .http1_title_case_headers(true)
                .http1_preserve_header_case(true)
                .build(https),
        })
    }
}

#[derive(Debug)]
pub struct WantsCa<C> {
    als: AddrListenerServer,
    client: Client<C>,
}

impl <C> ProxyBuilder<WantsCa<C>> {
    pub fn with_ca<CA: CertificateAuthority>(
        self,
        ca: CA
    ) -> ProxyBuilder<WantsHandlers<C, CA, NoopHandler, NoopHandler>> {
        ProxyBuilder(WantsHandlers {
            als: self.0.als,
            client: self.0.client,
            ca,
            http_handler: NoopHandler::new(),
            websocket_handler: NoopHandler::new(),
            websocket_connector: None,
        })
    }
}

pub struct WantsHandlers<C, CA, H, W> {
    als: AddrListenerServer,
    client: Client<C>,
    ca: CA,
    http_handler: H,
    websocket_handler: W,
    websocket_connector: Option<Connector>
}

impl<C, CA, H, W> ProxyBuilder<WantsHandlers<C, CA, H, W>> {

    pub fn with_http_handler<H2: HttpHandler>(
        self,
        http_handler: H2
    ) -> ProxyBuilder<WantsHandlers<C, CA, H2, W>> {
        ProxyBuilder(WantsHandlers {
            als: self.0.als,
            client: self.0.client,
            ca: self.0.ca,
            http_handler,
            websocket_handler: self.0.websocket_handler,
            websocket_connector: self.0.websocket_connector,
        })
    }

    pub fn with_websocket_handler<W2: WebSocketHandler>(
        self,
        handler: W2
    ) -> ProxyBuilder<WantsHandlers<C, CA, H, W2>> {
        ProxyBuilder((WantsHandlers {
            als: self.0.als,
            client: self.0.client,
            ca: self.0.ca,
            http_handler: self.0.http_handler,
            websocket_handler: handler,
            websocket_connector: self.0.websocket_connector,
        }))
    }

    pub fn build(self) -> Proxy<C, CA, H, W> {
        Proxy {
            als: self.0.als,
            client: self.0.client,
            ca: Arc::new(self.0.ca),
            http_handler: self.0.http_handler,
            websocket_handler: self.0.websocket_handler,
            websocket_connector: self.0.websocket_connector,
        }
    }
}