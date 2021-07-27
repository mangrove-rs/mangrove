use super::AnyResult;
use super::Endpoint;
use super::Request;
use super::Response;
use log::info;
use std::ffi::OsStr;
use std::path::PathBuf;
pub struct ServerFile {
  path: String,
}

impl ServerFile {
  pub fn new(path: String) -> Self {
    ServerFile { path }
  }
}

#[async_trait::async_trait]
impl Endpoint for ServerFile {
  async fn call(&self, req: Request) -> AnyResult<Response> {
    let file = req.get_param::<String>("file")?;
    let mut path = PathBuf::from(self.path.as_str());
    path.push(file.as_str());

    let ext = path.extension();

    info!("ext is {:?}", ext);
    let mut content_type = mime::TEXT_HTML_UTF_8.to_string();
    if ext == Some(OsStr::new("js")) {
      content_type = mime::TEXT_JAVASCRIPT.to_string();
    } else if ext == Some(OsStr::new("css")) {
      content_type = mime::TEXT_CSS_UTF_8.to_string()
    } else if ext == Some(OsStr::new("json")) {
      content_type = mime::JSON.to_string()
    } else if ext == Some(OsStr::new("svg")) {
      content_type = mime::SVG.to_string()
    } else if ext == Some(OsStr::new("xml")) {
      content_type = mime::TEXT_XML.to_string()
    } else if ext == Some(OsStr::new("woff")) {
      content_type = mime::FONT_WOFF.to_string()
    } else if ext == Some(OsStr::new("woff2")) {
      content_type = mime::FONT_WOFF2.to_string()
    } else if ext == Some(OsStr::new("jpeg")) {
      content_type = mime::JPEG.to_string()
    } else if ext == Some(OsStr::new("png")) {
      content_type = mime::PNG.to_string()
    } else if ext == Some(OsStr::new("pdf")) {
      content_type = mime::PDF.to_string()
    }
    let file = tokio::fs::read_to_string(&path).await?;
    Ok(Response::file(file, content_type).into())
  }
}
