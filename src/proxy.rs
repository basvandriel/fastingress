use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::Uri;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

use http_body_util::Empty;
use hyper::client::conn::http1::handshake;
use tokio::net::TcpStream;

use tokio::task::spawn;

use http_body_util::BodyExt;

use crate::constants::HTTP_PORT;

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

pub type R = Response<BoxBody<Bytes, hyper::Error>>;

pub async fn proxy_response(uri: Uri) -> Result<R, ErrorType> {
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(HTTP_PORT);

    let address = format!("{}:{}", host, port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    // Perform a TCP handshake
    let (mut sender, conn) = handshake::<TokioIo<TcpStream>, Empty<Bytes>>(io).await?;

    spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    // The authority of our URL will be the hostname of the httpbin remote
    let authority = uri.authority().unwrap().clone();

    // Create an HTTP request with an empty body and a HOST header
    let req = Request::builder()
        .uri(uri)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let res = sender.send_request(req).await?;
    let mapped = res.map(|i| i.boxed());

    return Ok(mapped);
}