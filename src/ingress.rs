use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::Request;
use hyper::Response;
use hyper::StatusCode;
use hyper::Uri;
use rand::distributions::{Alphanumeric, DistString};
use std::time::Instant;

use crate::logger::Logger;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::route_entry::RouteEntry;
use crate::routedebugger::RouteDebugger;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::uri_resolver::InClusterServiceURLResolver;
use crate::uri_resolver::ProxiedServiceURLResolver;
use crate::uri_resolver::UrlResolver;

type RQ = Request<Incoming>;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler {
    routes: Vec<RouteEntry>,
}

impl IngressRequestHandler {
    pub fn new(routes: Vec<RouteEntry>) -> Self {
        Self { routes }
    }

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

    fn matchpath(&self, path: &str) -> Option<&RouteEntry> {
        self.routes.iter().find(|r| r.route == path)
    }

    fn resolve_url(&self, original_uri: &Uri) -> Option<Uri> {
        let logger = Logger {};
        RouteDebugger::new(logger).debug(&self.routes);

        let resolved = self.matchpath(original_uri.path());

        if resolved.is_none() {
            return None;
        }
        let servicelocation = resolved.unwrap();

        let url = self
            .build_url_resolver(original_uri.clone())
            .resolve(servicelocation);

        Some(url.expect("URI should be there"))
    }

    fn notfound(&self, body: BoxBody<Bytes, hyper::Error>) -> Result<R, ErrorType> {
        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap();

        Ok(response)
    }

    pub async fn proxy_to_service(&self, request: RQ) -> Result<R, ErrorType> {
        let logger: Logger = Logger {};
        let start = Instant::now();

        let request_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

        logger.info(&format!(
            "Incoming request (\"{}\"): {} \"{}\"",
            request_id,
            request.method(),
            request.uri(),
        ));
        let url = self.resolve_url(request.uri());

        if url.is_none() {
            logger.info("No suiting routes found for request. Aborting");

            let body = BoxBody::<Bytes, hyper::Error>::new(request);
            return self.notfound(body);
        }

        // TODO use everything from original request (method, body, ...)
        let result = proxy_response(url.unwrap()).await?;

        logger.info(&format!(
            "Request \"{}\" finished - took {}ms",
            request_id,
            start.elapsed().as_millis()
        ));
        Ok(result)
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
