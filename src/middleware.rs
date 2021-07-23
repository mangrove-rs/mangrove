use super::AnyResult;
use super::Next;
use super::Request;
use super::Response;
use std::future::Future;

#[async_trait::async_trait]
pub trait Middleware: Send + Sync + 'static {
  async fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> AnyResult<Response>;
  fn name(&self) -> &str {
    std::any::type_name::<Self>()
  }
}

#[async_trait::async_trait]
impl<F, Fut> Middleware for F
where
  F: Fn(Request, Next<'_>) -> Fut + Send + Sync + 'static,
  Fut: Future<Output = AnyResult<Response>> + Send + 'static,
{
  async fn handle<'a>(&'a self, req: Request, next: Next<'a>) -> AnyResult<Response> {
    (self)(req, next).await
  }
}
