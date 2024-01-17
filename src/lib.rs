pub mod service_resolver;

use std::convert::Infallible;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use tokio::task::spawn;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub async fn eventloop(listener: &TcpListener) -> Result<(), std::io::Error> {
    let (stream, _) = listener.accept().await?;

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    spawn(async move {
        // Finally, we bind the incoming connection to our `hello` service
        if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
            .serve_connection(io, service_fn(hello))
            .await
        {
            println!("Error serving connection: {:?}", err);
        }
    });
    Ok(())
}
