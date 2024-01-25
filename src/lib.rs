pub mod api_resolver;
pub mod constants;
pub mod ingress;
pub mod ipfinder;
mod logger;
pub mod proxy;
pub mod service_resolver;
pub mod uri_resolver;
pub mod utils;

use hyper::body::Incoming;
use hyper::server::conn::http1::Builder as HTTPBuilder;
use hyper::Request;

use crate::proxy::R;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use ingress::{ErrorType, IngressRequestHandler};

use tokio::net::TcpListener;

use tokio::task::spawn;

async fn hello(req: Request<Incoming>) -> Result<R, ErrorType> {
    return IngressRequestHandler.proxy_to_service(req).await;
}

pub async fn accept_connection(listener: &TcpListener) -> () {
    let (stream, _) = listener.accept().await.expect("No");
    let io = TokioIo::new(stream);

    let service = service_fn(hello);

    spawn(async move {
        if let Err(err) = HTTPBuilder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    });
}
