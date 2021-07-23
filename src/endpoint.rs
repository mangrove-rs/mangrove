use super::AnyResult;
use super::Request;
use super::Response;
use std::future::Future;

#[async_trait::async_trait]
pub trait Endpoint: Send + Sync + 'static {
  async fn call(&self, req: Request) -> AnyResult<Response>;
}

#[async_trait::async_trait]
impl<F: Send + Sync + 'static, Fut, Res> Endpoint for F
where
  F: Fn(Request) -> Fut,
  Fut: Future<Output = AnyResult<Res>> + Send + 'static,
  Res: Into<Response> + 'static,
{
  async fn call(&self, req: Request) -> AnyResult<Response> {
    let res = self(req).await?;
    Ok(res.into())
  }
}
