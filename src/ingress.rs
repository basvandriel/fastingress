use hyper::body::Incoming;
use hyper::Request;
use hyper::Uri;
use k8s_openapi::api::networking::v1::HTTPIngressPath;
use k8s_openapi::api::networking::v1::IngressRule;
use kube::api::Log;
use rand::distributions::{Alphanumeric, DistString};
use std::time::Instant;

use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::service_resolver::KubeServiceLocation;
use crate::types::RouteMap;
use crate::uri_resolver::InClusterServiceURLResolver;
use crate::uri_resolver::ProxiedServiceURLResolver;
use crate::uri_resolver::UrlResolver;

type RQ = Request<Incoming>;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler;

impl IngressRequestHandler {
    fn log_request(&self, duration_ms: u128, request_id: &str) {
        let message = format!(
            "Request \"{}\" finished - took {}ms",
            request_id, duration_ms
        );

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

    fn debug_route(&self, route: &IngressRule, logger: Logger) {
        let http = route.http.as_ref().unwrap();
        let paths: Vec<HTTPIngressPath> = http.paths.clone();

        for pathobj in paths.iter() {
            let path = pathobj.path.as_ref().unwrap();
            let service = pathobj.backend.service.as_ref().unwrap();
            let port = service.port.as_ref().unwrap().number.unwrap();

            logger.info(&format!(
                "Routing {} -> {} on port {}",
                path, service.name, port
            ));
        }
    }
    fn debug_routes(&self, x: &RouteMap, logger: Logger) {
        for route in x.iter() {
            self.debug_route(route, logger);
        }
    }

    pub async fn proxy_to_service(&self, request: RQ, x: RouteMap) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};
        let start = Instant::now();

        self.debug_routes(&x, logger);
        let request_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

        let logmsg = format!(
            "Incoming request (\"{}\"): {} \"{}\"",
            request_id,
            request.method(),
            request.uri(),
        );
        logger.info(&logmsg);

        let url = self.resolve_url(request.uri()).await;

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url).await?;

        self.log_request(start.elapsed().as_millis(), &request_id);
        return Ok(result);
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
