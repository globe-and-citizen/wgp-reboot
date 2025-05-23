use async_trait::async_trait;
use bytes::Bytes;
use std::net::ToSocketAddrs;
use log::{info};
use pingora::upstreams::peer::HttpPeer;
use pingora::{Result};
use pingora::http::{StatusCode};
use pingora::proxy::{ProxyHttp, Session};
use crate::proxy_handler::ProxyHandler;

pub struct EchoProxy<T> {
    addr: std::net::SocketAddr,
    handler: ProxyHandler<T>,
}

impl<T: Sync> EchoProxy<T> {
    pub(crate) fn new(upstream_host: String, upstream_port: u16, handler: ProxyHandler<T>) -> Self {
        let addr = (upstream_host, upstream_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        EchoProxy { addr, handler }
    }
}

#[async_trait]
impl<T: std::marker::Sync> ProxyHttp for EchoProxy<T> {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX { () }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer: Box<HttpPeer> = Box::new(HttpPeer::new(self.addr, false, self.addr.ip().to_string()));
        Ok(peer)
    }

    async fn request_filter(&self, session: &mut Session, _ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        let mut response_body_bytes = Vec::new();

        // validate request
        let mut response_status = self.handler.validate_request(session);
        if response_status == StatusCode::OK {
            // handle request
            match self.handler.handle_request(session).await {
                Ok(res) => {
                    response_body_bytes = res
                }
                Err(err) => {
                    // todo create error response
                    response_status = StatusCode::BAD_REQUEST;
                }
            }
        }

        // convert json response to vec
        ProxyHandler::<T>::set_headers(response_status, &response_body_bytes, session).await?;
        session.write_response_body(Some(Bytes::from(response_body_bytes)), true).await?;

        Ok(true)
    }

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        // access log
        info!("{} response code: {response_code}", self.request_summary(session, ctx));
    }
}
