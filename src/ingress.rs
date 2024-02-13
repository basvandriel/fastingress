use hyper::body::Incoming;
use hyper::Request;
use hyper::Uri;
use k8s_openapi::api::networking::v1::HTTPIngressPath;
use k8s_openapi::api::networking::v1::IngressRule;
use rand::distributions::{Alphanumeric, DistString};
use std::time::Instant;

use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::route_entry::RouteEntry;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::service_resolver::KubeServiceLocation;
use crate::types::RouteMap;
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

    fn resolve_rule_entries(&self, route: &IngressRule) -> Vec<RouteEntry> {
        let http = route.http.as_ref().unwrap();
        let paths: Vec<HTTPIngressPath> = http.paths.clone();

        let mut entries: Vec<RouteEntry> = vec![];

        for pathobj in paths.iter() {
            let path = pathobj.path.as_ref().unwrap();
            let service = pathobj.backend.service.as_ref().unwrap();
            let port = service.port.as_ref().unwrap().number.unwrap();

            let x = RouteEntry {
                host: "localhost".to_string(),
                port,
                route: path.to_owned(),
                service: service.name.to_owned(),
            };
            entries.push(x);
        }

        return entries;
    }

    fn resolve_route_entries(&self, routemap: &RouteMap) -> Vec<RouteEntry> {
        let entries: Vec<RouteEntry> = routemap
            .iter()
            .flat_map(|route| self.resolve_rule_entries(route))
            .collect();

        return entries;
    }

    fn debug_routes(&self, route_entries: &Vec<RouteEntry>, logger: Logger) {
        logger.info("Available routes:");
        println!("");
        println!(
            "{0: <20} | {1: <10} | {2: <20} | {3: <10}",
            "host", "route", "service", "port"
        );

        for entry in route_entries.iter().clone() {
            println!(
                "{0: <20} | {1: <10} | {2: <20} | {3: <10}",
                entry.host, entry.route, entry.service, entry.port
            );
        }
        println!("");
    }

    pub async fn proxy_to_service(&self, request: RQ, x: RouteMap) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};
        let start = Instant::now();

        let entries = self.resolve_route_entries(&x);
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
        return Ok(result);
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
