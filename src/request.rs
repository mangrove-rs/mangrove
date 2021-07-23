use super::HyperRequest;
use super::Method;
use route_recognizer::Params;
use std::net::SocketAddr;
pub struct Request {
  pub(crate) inner: HyperRequest,
  pub(crate) params: Params,
  pub(crate) remote_addr: Option<SocketAddr>,
}

impl Request {
  pub fn new(req: HyperRequest, remote_addr: Option<SocketAddr>) -> Self {
    Request {
      inner: req,
      params: Params::new(),
      remote_addr,
    }
  }
  pub fn get_remote_addr(&self) -> Option<SocketAddr> {
    self.remote_addr
  }
  pub fn get_method(&self) -> &Method {
    self.inner.method()
  }
}

impl From<HyperRequest> for Request {
  fn from(req: HyperRequest) -> Self {
    Request::new(req, None)
  }
}
