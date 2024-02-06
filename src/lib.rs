pub mod api_resolver;
pub mod constants;
pub mod ingress;
pub mod ipfinder;
pub mod kube_api_structs;
pub mod logger;
pub mod paths;
pub mod proxy;
pub mod service_resolver;
pub mod uri_resolver;
pub mod utils;

// use futures::channel::mpsc::Receiver;
use hyper::body::Incoming;
use hyper::server::conn::http1::Builder as HTTPBuilder;
use hyper::Request;
use k8s_openapi::api::networking::v1::IngressRule;
use logger::Logger;
use tokio::sync::mpsc::Receiver;

use crate::proxy::R;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use ingress::{ErrorType, IngressRequestHandler};

use tokio::net::TcpListener;
pub mod ingress_resource_json_parser;

pub mod api_watcher;
pub mod ingress_resource_resolver;

use tokio::spawn;

async fn hello(req: Request<Incoming>) -> Result<R, ErrorType> {
    return IngressRequestHandler.proxy_to_service(req).await;
}

pub async fn accept_connection(
    listener: &TcpListener,
    logger: Logger,
    rx: &mut Receiver<IngressRule>,
) -> () {
    let addr = listener.local_addr().unwrap();
    logger.info(format!("Listening for new TCP connections on http://{}", addr).as_str());

    while let Some(_message) = rx.recv().await {
        logger.info("Received Ingress Rule via MPSC");
    }
    let (stream, _) = listener.accept().await.expect("No");

    spawn(async move {
        let io = TokioIo::new(stream);
        let service = service_fn(hello);
        if let Err(err) = HTTPBuilder::new().serve_connection(io, service).await {
            println!("Error serving connection: {:?}", err);
        }
    });
}
