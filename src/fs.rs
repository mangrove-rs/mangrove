use super::AnyResult;
use super::Request;
use super::Response;
async fn file(_req: Request) -> AnyResult {
  Ok("file".into())
}

async fn dir(_req: Request) -> AnyResult {
  Ok("dir".into())
}
