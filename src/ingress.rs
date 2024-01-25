use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;
use hyper::Uri;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use crate::logger::log_request;
use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::service_resolver::KubeServiceLocation;
use crate::uri_resolver::InClusterServiceURLResolver;
use crate::uri_resolver::ProxiedServiceURLResolver;
use crate::uri_resolver::UrlResolver;

type RQ = Request<Incoming>;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler;

impl IngressRequestHandler {
    fn build_url_resolver(original_uri: Uri) -> Box<dyn UrlResolver> {
        if running_in_kubernetes_cluster() {
            return Box::new(InClusterServiceURLResolver {
                original_url: original_uri,
            });
        }
        return Box::new(ProxiedServiceURLResolver {
            original_url: original_uri,
        });
    }

    async fn resolve_url(original_uri: &Uri) -> Uri {
        let loc = KubeServiceLocation {
            namespace: String::from("default"),
            name: String::from("nginx-service"),
            port: 80,
        };
        let url = Self::build_url_resolver(original_uri.clone()).resolve(&loc);

        return url.expect("URI should be there");
    }

    async fn proxy_to_service(request: RQ) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};

        let start = Instant::now();
        let url = Self::resolve_url(request.uri()).await;
        logger.info(format!("Routing to URI: {}", url).as_str());

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url).await?;

        log_request(request, start.elapsed().as_millis());

        return Ok(result);
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;

impl Service<RQ> for IngressRequestHandler {
    type Response = R;
    type Error = ErrorType;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: RQ) -> Self::Future {
        let res = Self::proxy_to_service(req);
        Box::pin(res)
    }
}
