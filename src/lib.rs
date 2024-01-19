pub mod constants;
mod ingress;
mod logger;
pub mod proxy;
pub mod service_resolver;

use hyper::server::conn::http1::Builder as HTTPBuilder;

use hyper_util::rt::TokioIo;
use ingress::IngressRequestHandler;

use tokio::net::TcpListener;

use tokio::task::spawn;

pub async fn accept_connection(listener: &TcpListener) -> () {
    let (stream, _) = listener.accept().await.expect("No");
    let io = TokioIo::new(stream);

    let service = IngressRequestHandler {};

    spawn(async move {
        if let Err(err) = HTTPBuilder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    });
}
