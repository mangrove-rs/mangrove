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
  info!("router {:?}", router);

  let server = Server::new(router);
  let addr = "127.0.0.1:3039".parse()?;
  info!("addr {:?}", addr);
  server.run(&addr).await?;
  Ok(())
}

mod controller {
  use mangrove::AnyResult;
  use mangrove::Request;

  pub async fn hello(_req: Request) -> AnyResult {
    Ok("HELLO".into())
  }
}
