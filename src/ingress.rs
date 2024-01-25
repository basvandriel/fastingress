use hyper::body::Incoming;
use hyper::Request;
use hyper::Uri;
use std::time::Instant;

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
    fn log_request<T>(&self, request: Request<T>, duration_ms: u128) {
        let method = request.method();
        let path = request.uri().path();

        let message = format!("{} \"{}\" - took {}ms", method, path, duration_ms);

        let logger = Logger {};
        logger.info(&message);
    }

    fn build_url_resolver(&self, original_uri: Uri) -> Box<dyn UrlResolver> {
        if running_in_kubernetes_cluster() {
            return Box::new(InClusterServiceURLResolver {
                original_url: original_uri,
            });
        }
        return Box::new(ProxiedServiceURLResolver {
            original_url: original_uri,
        });
    }

    async fn resolve_url(&self, original_uri: &Uri) -> Uri {
        let loc = KubeServiceLocation {
            namespace: String::from("default"),
            name: String::from("nginx-service"),
            port: 80,
        };
        let url = self.build_url_resolver(original_uri.clone()).resolve(&loc);

        return url.expect("URI should be there");
    }

    pub async fn proxy_to_service(&self, request: RQ) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};

        let start = Instant::now();
        let url = self.resolve_url(request.uri()).await;
        logger.info(format!("Routing to URI: {}", url).as_str());

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url).await?;

        self.log_request(request, start.elapsed().as_millis());
        return Ok(result);
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
