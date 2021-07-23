use super::Response;
use super::Router;
use super::Error;
use super::HyperRequest;
use hyper::service::{make_service_fn, service_fn};
use std::net::SocketAddr;
use std::sync::Arc;
pub struct Server {
  pub router: Router,
}
impl Server {
  pub fn new(router: Router) -> Self {
    Server { router }
  }
  pub async fn run(self, addr: &SocketAddr) -> Result<(), Error> {
    let router = Arc::new(self.router);
    let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
      let remote_addr = conn.remote_addr();
      let router = router.clone();
      async move {
        Ok::<_, Error>(service_fn(move |req: HyperRequest| {
          let router = router.clone();
          async move {
            match router.dispatch(req, Some(remote_addr)).await {
              Ok(resp) => Ok::<_, Error>(resp.into()),
              Err(e) => {
                println!("e {:?}", e);
                Ok::<_, Error>(
                  Response::with_status(hyper::StatusCode::from_u16(500).unwrap(), e.to_string())
                    .into(),
                )
              }
            }
          }
        }))
      }
    });

    let server = hyper::Server::bind(addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
      eprintln!("server error: {}", e);
    }
    Ok(())
  }
}

