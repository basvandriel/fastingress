use crate::ingress::ErrorType;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::header::HOST;
use hyper::Uri;
use hyper::{Request, Response};

use crate::utils;

pub type R = Response<BoxBody<Bytes, hyper::Error>>;

pub async fn proxy_response(uri: Uri) -> Result<R, ErrorType> {
    type BodyType = Empty<Bytes>;

    let mut sender = utils::handshake_url::<BodyType>(&uri).await?;
    let authority = uri.authority().unwrap().clone();

    // TODO This needs the path and headers and body as well
    let req = Request::builder()
        .uri(uri)
        .header(HOST, authority.as_str())
        .body(Empty::new())?;

    let res = sender.send_request(req).await?;
    let mapped = res.map(|i| i.boxed());

    Ok(mapped)
}
