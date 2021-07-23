use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("hyper error")]
  HyperError(#[from] hyper::Error),
  #[error("http error")]
  HttpError(#[from] hyper::http::Error),
  #[error("serde error")]
  JsonError(#[from] serde_json::Error),
  #[error("query error")]
  QueryError(#[from] serde_urlencoded::de::Error),
  #[error("query error")]
  QSError(#[from] serde_qs::Error),
  #[error("io error")]
  IoError(#[from] std::io::Error),
  #[error("error message: `{0}`")]
  Message(String),
  #[error("unwrap none error")]
  UnwrapNone(),
  #[error("unknown error")]
  Unknown,
}
