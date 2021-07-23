use crate::endpoint::Endpoint;
use super::Error;
use super::Response;

pub type HyperResponse = hyper::Response<hyper::Body>;
pub type HyperRequest = hyper::Request<hyper::Body>;
pub type AnyResult<T = Response> = anyhow::Result<T, anyhow::Error>;
pub type Result<T = Response> = std::result::Result<T, Error>;
pub type DynEndpoint = dyn Endpoint;
