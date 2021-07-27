extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use mangrove::AnyResult;
use mangrove::Router;
use mangrove::Server;

#[tokio::main]
async fn main() -> AnyResult<()> {
  pretty_env_logger::init();

  let mut router = Router::new();
  router.get("/", |_| async { Ok("Hello") });
  router.get("/hello", controller::hello);
  router.get("/file", controller::file);
  router.statics("/statics/:file", "examples");
  info!("router {:?}", router);

  let server = Server::new(router);
  let addr = "127.0.0.1:3039".parse()?;
  info!("addr {:?}", addr);
  server.run(&addr).await?;
  Ok(())
}

mod controller {
  use std::ffi::OsStr;
  use std::path::Path;

  use mangrove::AnyResult;
  use mangrove::Request;
  use mangrove::Response;

  pub async fn hello(_req: Request) -> AnyResult {
    Ok("HELLO".into())
  }
  pub async fn file(_req: Request) -> AnyResult {
    let file = tokio::fs::read_to_string("./examples/index.html").await?;
    Ok(Response::html(file).into())
  }
  pub async fn statics(req: Request) -> AnyResult {
    let file = req.get_param::<String>("file")?;
    let path = format!("./examples/{}", file);
    let path = Path::new(path.as_str());
    let ext = path.extension();
    info!("ext is {:?}", ext);
    let mut content_type = mime::TEXT_HTML_UTF_8.to_string();
    if ext == Some(OsStr::new("js")) {
      content_type = mime::TEXT_JAVASCRIPT.to_string();
    } else if ext == Some(OsStr::new("css")) {
      content_type = mime::TEXT_CSS_UTF_8.to_string()
    }
    let file = tokio::fs::read_to_string(&path).await?;
    Ok(Response::file(file, content_type).into())
  }
}
