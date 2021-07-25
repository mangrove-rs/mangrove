use super::HyperResponse;
use super::StatusCode;
use serde::Serialize;
pub struct Response {
  pub(crate) inner: HyperResponse,
}

impl Response {
  pub fn new(res: HyperResponse) -> Self {
    Response { inner: res }
  }
  pub fn get_status(&self) -> StatusCode {
    self.inner.status()
  }

  pub fn json<T: Serialize + Send>(val: T) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::APPLICATION_JSON.to_string(),
      )
      .status(StatusCode::OK)
      .body(hyper::Body::from(serde_json::to_string(&val).unwrap()))
      .unwrap()
      .into()
  }
  
  pub fn html(val: String) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::HTML.to_string()
      )
      .status(StatusCode::OK)
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }
  pub fn file(val: String, content_type: String) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        content_type
      )
      .status(StatusCode::OK)
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }

  pub fn download(val: String) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::HTML.to_string()
      )
      .status(StatusCode::OK)
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }

  pub fn with_status(status: hyper::StatusCode, val: String) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::TEXT_PLAIN_UTF_8.to_string(),
      )
      .status(status)
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }
}

impl From<HyperResponse> for Response {
  fn from(response: HyperResponse) -> Self {
    Response { inner: response }
  }
}
impl Into<HyperResponse> for Response {
  fn into(self) -> HyperResponse {
    let Response { inner } = self;
    inner
  }
}

impl From<String> for Response {
  fn from(val: String) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::TEXT_PLAIN_UTF_8.to_string(),
      )
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }
}
impl From<&'static str> for Response {
  fn from(val: &'static str) -> Self {
    hyper::http::Response::builder()
      .header(
        hyper::header::CONTENT_TYPE,
        mime::TEXT_PLAIN_UTF_8.to_string(),
      )
      .body(hyper::Body::from(val))
      .unwrap()
      .into()
  }
}
