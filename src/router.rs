use super::AnyResult;
use super::Middleware;
use super::Next;
use super::Request;
use super::Response;
use super::HyperRequest;
use super::Endpoint;
use super::DynEndpoint;
use route_recognizer::Params;
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use super::fs::ServerFile;
// Router

async fn default_handler(_req: Request) -> AnyResult<Response> {
  Ok(Response::with_status(
    hyper::StatusCode::from_u16(404).unwrap(),
    "handler not found".to_string(),
  ))
}


pub struct Router {
  pub middlewares: Vec<Arc<dyn Middleware>>,
  pub not_found_handler: Box<DynEndpoint>,
  pub prefix: Option<String>,
  pub routes: HashMap<hyper::Method, route_recognizer::Router<Box<DynEndpoint>>>,
}

impl Router {
  pub fn new() -> Self {
    Router {
      middlewares: Vec::new(),
      not_found_handler: Box::new(&default_handler),
      prefix: Some("".to_owned()),
      routes: HashMap::new(),
    }
  }

  pub fn set_prefix(&mut self, prefix: &str) -> &mut Self {
    self.prefix = Some(prefix.to_owned());
    self
  }

  pub fn set_not_found_handler(&mut self, ep: impl Endpoint) -> &mut Self {
    self.not_found_handler = Box::new(ep);
    self
  }

  pub fn middleware(&mut self, middleware: impl Middleware) -> &mut Self {
    self.middlewares.push(Arc::new(middleware));
    self
  }

  pub fn direct(&mut self, method: hyper::Method, route: &str, dest: impl Endpoint) {
    self
      .routes
      .entry(method)
      .or_insert_with(route_recognizer::Router::new)
      .add(route, Box::new(dest));
  }

  pub fn at(&mut self, method: hyper::Method, route: &str, dest: impl Endpoint) {
    let mut path = String::from("");
    if let Some(prefix) = &self.prefix {
      path.push_str(prefix.as_str());
    }
    path.push_str(route);
    self
      .routes
      .entry(method)
      .or_insert_with(route_recognizer::Router::new)
      .add(path.as_str(), Box::new(dest));
  }

  pub fn get(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::GET, route, dest);
  }

  pub fn post(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::POST, route, dest);
  }

  pub fn delete(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::DELETE, route, dest);
  }

  pub fn patch(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::PATCH, route, dest);
  }

  pub fn put(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::PUT, route, dest);
  }

  pub fn options(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::OPTIONS, route, dest);
  }

  pub fn head(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::HEAD, route, dest);
  }

  pub fn connect(&mut self, route: &str, dest: impl Endpoint) {
    self.at(hyper::Method::CONNECT, route, dest);
  }

  pub fn statics(&mut self, route: &str, path: &str) {
    self.at(hyper::Method::GET, route, ServerFile::new(path.to_string()))
  }

  pub async fn dispatch(
    &self,
    req: HyperRequest,
    remote_addr: Option<SocketAddr>,
  ) -> AnyResult<Response> {
    let method = req.method();
    let path = req.uri().path();

    let mut params = Params::new();
    let endpoint = match self.routes.get(method) {
      Some(route) => match route.recognize(path) {
        Ok(m) => {
          params = m.params().to_owned();
          &***m.handler()
        }
        Err(_e) => &*self.not_found_handler,
      },
      None => &*self.not_found_handler,
    };

    let mut request = Request::new(req, remote_addr);
    request.params = params;
    let next = Next {
      endpoint: endpoint,
      middlewares: &self.middlewares,
    };
    next.run(request).await
  }

}

impl std::fmt::Debug for Router {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Router{{ middlewares: (length: {}) }}, prefix {:?}",
      self.middlewares.len(),
      self.prefix
    )
  }
}
