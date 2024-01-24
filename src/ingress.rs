use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;
use hyper::Uri;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use crate::api_resolver::resolve_in_cluster_service_uri;
use crate::logger;
use crate::logger::log_request;
use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::service_resolver::build_service_proxy_url;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::service_resolver::KubeServiceLocation;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler;

impl IngressRequestHandler {
    const LOGGER: Logger = Logger {};

    async fn resolve_url(original_uri: &Uri) -> Uri {
        let loc = KubeServiceLocation {
            namespace: String::from("default"),
            name: String::from("nginx-service"),
            port: 80,
        };
        let clusterip = resolve_in_cluster_service_uri(&loc).expect("!");
        Self::LOGGER.info(format!("IP in cluster {}", clusterip).as_str());

        let url: Uri;
        if running_in_kubernetes_cluster() {
            url = clusterip;
        } else {
            url = build_service_proxy_url(&loc, &original_uri);
        }
        return url;
    }

    async fn proxy_to_service(request: Request<Incoming>) -> Result<R, ErrorType> {
        let start = Instant::now();
        let url = Self::resolve_url(request.uri()).await;

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url).await?;

        log_request(request, start.elapsed().as_millis());

        return Ok(result);
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;

impl Service<Request<Incoming>> for IngressRequestHandler {
    type Response = R;
    type Error = ErrorType;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let res = Self::proxy_to_service(req);
        Box::pin(res)
    }
}
