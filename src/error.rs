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
  #[error("invalid param {name:?} as {expected:?}, {err:?}")]
  InvalidParam {
    name: String,
    expected: &'static str,
    err: String,
  },
  #[error("missing url param {name:?}")]
  MissingParam { name: String },
  #[error("error message: `{0}`")]
  Message(String),
  #[error("unwrap none error")]
  UnwrapNone(),
  #[error("unknown error")]
  Unknown,
}
pub fn missing_param(name: impl ToString) -> Error {
  Error::MissingParam {
    name: name.to_string(),
  }
}
pub fn invalid_param(
  name: impl ToString,
  expected: &'static str,
  err: impl std::error::Error,
) -> Error {
  Error::InvalidParam {
    name: name.to_string(),
    expected,
    err: err.to_string(),
  }
}
