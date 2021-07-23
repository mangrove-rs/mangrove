use std::sync::Arc;

use super::AnyResult;
use super::DynEndpoint;
use super::Middleware;
use super::Request;
use super::Response;

// Next
#[allow(missing_debug_implementations)]
pub struct Next<'a> {
  pub(crate) endpoint: &'a DynEndpoint,
  pub(crate) middlewares: &'a [Arc<dyn Middleware>],
}

impl<'a> Next<'a> {
  pub async fn run(mut self, req: Request) -> AnyResult<Response> {
    if let Some((current, next)) = self.middlewares.split_first() {
      self.middlewares = next;
      current.handle(req, self).await
    } else {
      self.endpoint.call(req).await
    }
  }
}
