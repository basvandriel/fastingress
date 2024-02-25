use hyper::body::Incoming;
use hyper::Request;
use hyper::Uri;
use rand::distributions::{Alphanumeric, DistString};
use std::time::Instant;

use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::route_entry::RouteEntry;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::service_resolver::KubeServiceLocation;
use crate::uri_resolver::InClusterServiceURLResolver;
use crate::uri_resolver::ProxiedServiceURLResolver;
use crate::uri_resolver::UrlResolver;

type RQ = Request<Incoming>;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler {}

impl IngressRequestHandler {
    fn build_url_resolver(&self, original_uri: Uri) -> Box<dyn UrlResolver> {
        if running_in_kubernetes_cluster() {
            return Box::new(InClusterServiceURLResolver {
                original_url: original_uri,
            });
        }
        Box::new(ProxiedServiceURLResolver {
            original_url: original_uri,
        })
    }

    async fn resolve_url(&self, original_uri: &Uri) -> Uri {
        let loc = KubeServiceLocation {
            namespace: String::from("default"),
            name: String::from("nginx-service"),
            port: 80,
        };
        let url = self.build_url_resolver(original_uri.clone()).resolve(&loc);

        url.expect("URI should be there")
    }

    fn debug_routes(&self, route_entries: &[RouteEntry], logger: Logger) {
        logger.info("Available routes:");
        println!();
        println!(
            "{0: <35} | {1: <15} | {2: <10} | {3: <20} | {4: <10}",
            "ingress_name", "host", "route", "service", "port"
        );

        for entry in route_entries.iter().clone() {
            println!(
                "{0: <35} | {1: <15} | {2: <10} | {3: <20} | {4: <10}",
                entry.ingress_name, entry.host, entry.route, entry.service, entry.port
            );
        }
        println!();
    }

    pub async fn proxy_to_service(&self, request: RQ, x: Vec<RouteEntry>) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};
        let start = Instant::now();

        let entries = x;
        self.debug_routes(&entries, logger);

        let request_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

        logger.info(&format!(
            "Incoming request (\"{}\"): {} \"{}\"",
            request_id,
            request.method(),
            request.uri(),
        ));

        let url = self.resolve_url(request.uri()).await;

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url).await?;

        logger.info(&format!(
            "Request \"{}\" finished - took {}ms",
            request_id,
            start.elapsed().as_millis()
        ));
        Ok(result)
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
