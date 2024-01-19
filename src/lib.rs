pub mod constants;
pub mod proxy;
pub mod service_resolver;

use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Uri};
use hyper_util::rt::TokioIo;
use proxy::{proxy_response, R};
use service_resolver::build_service_proxy_url;
use tokio::net::TcpListener;

use tokio::task::spawn;

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

pub async fn hello(_request: Request<Incoming>) -> Result<R, ErrorType> {
    let url = build_service_proxy_url("nginx-service", 80).parse::<Uri>()?;
    let result = proxy_response(url).await?;

    println!("Handled response!");
    Ok(result)
}

pub async fn accept_connection(listener: &TcpListener) -> Result<(), std::io::Error> {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    spawn(async move {
        if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn(hello))
            .await
        {
            println!("Error serving connection: {:?}", err);
        }
    });
    Ok(())
}
