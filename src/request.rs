use super::error::{invalid_param, missing_param, Error};
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
  pub fn get_param<T>(&self, param: &str) -> Result<T, Error>
  where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::error::Error,
  {
    match self.params.find(param) {
      Some(param) => param
        .parse()
        .map_err(|e| invalid_param(param, std::any::type_name::<T>(), e)),
      None => Err(missing_param(param)),
    }
  }
}

impl From<HyperRequest> for Request {
  fn from(req: HyperRequest) -> Self {
    Request::new(req, None)
  }
}
