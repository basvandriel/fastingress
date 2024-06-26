use std::env;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use fastingress::constants::DEFAULT_LISTENING_PORT;
use fastingress::ingress_watcher::watcher::APIListener;
use fastingress::kube_client_resolver::KubeClientResolver;
use fastingress::logger::Logger;
use fastingress::request_handler::Svc;
use fastingress::route_entry::RouteEntry;
use fastingress::types::Arced;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use tokio::spawn;

fn resolve_ip() -> Ipv4Addr {
    let hostout = env::var("HOST_OUT").ok();

    if hostout.is_some() && hostout.unwrap().eq("1") {
        println!("INFO: Detected HOST_OUT=1 in environment");
        return Ipv4Addr::UNSPECIFIED;
    }
    Ipv4Addr::LOCALHOST
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let logger = Logger {};

    let kubeclient = KubeClientResolver::new(logger)
        .resolve()
        .await
        .expect("Kubernetes connection should be active");

    let routes: Arced<Vec<RouteEntry>> = Arc::new(Mutex::new(vec![]));
    let routes_clone = routes.clone();

    spawn(async move {
        let listener = APIListener { logger, routes };

        // Hand ownership to the API listener
        listener.listen(kubeclient).await;
    });

    let address = SocketAddr::from((resolve_ip(), DEFAULT_LISTENING_PORT));
    let listener = TcpListener::bind(address).await?;

    let svc = Svc {
        logger,
        routes_clone,
    };

    logger.info(&format!(
        "Listening for new TCP connections on http://{}",
        address
    ));

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        let svc_clone = svc.clone();
        // Open a thread for every connection we get.
        // this is needed so we can handle more requests at once
        spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}
