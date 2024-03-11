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
use crate::routematching::matcher::RouteMatcher;
use crate::routematching::prefixmatcher::PrefixRouteMatcher;
use crate::service_resolver::running_in_kubernetes_cluster;
use crate::urlresolving::incluster_url_resolver::InClusterServiceURLResolver;
use crate::urlresolving::proxied_url_resolver::ProxiedServiceURLResolver;
use crate::urlresolving::UrlResolver;
type RQ = Request<Incoming>;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler {
    routes: Vec<RouteEntry>,
    logger: Logger,
}

impl IngressRequestHandler {
    pub fn new(routes: Vec<RouteEntry>) -> Self {
        Self {
            routes,
            logger: Logger {},
        }
    }
    fn build_url_resolver(&self, original_url: Uri) -> Box<dyn UrlResolver> {
        if running_in_kubernetes_cluster() {
            return Box::new(InClusterServiceURLResolver { original_url });
        }
        Box::new(ProxiedServiceURLResolver { original_url })
    }
    fn find_routematcher(&self) -> Box<dyn RouteMatcher> {
        let matcher = PrefixRouteMatcher::new(self.routes.clone());
        Box::new(matcher)
    }
    fn resolve_url(&self, original_uri: &Uri) -> Option<Uri> {
        RouteDebugger::new(self.logger).debug(&self.routes);

        let matcher = self.find_routematcher();
        let resolved = matcher.find(original_uri.path());

        // https://rust-lang.github.io/rust-clippy/master/index.html#/question_mark
        resolved?;

        let servicelocation = resolved.unwrap();

        let url = self
            .build_url_resolver(original_uri.clone())
            .resolve(servicelocation);

        Some(url.expect("URI should be there"))
    }

    fn notfound(&self, body: BoxBody<Bytes, hyper::Error>) -> Result<R, ErrorType> {
        self.logger.info("No routes found for request. Aborting");

        let response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap();

        Ok(response)
    }

    pub async fn proxy_to_service(&self, request: RQ) -> Result<R, ErrorType> {
        let start = Instant::now();
        let request_id = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);

        self.logger.info(&format!(
            "Incoming request (\"{}\"): {} \"{}\"",
            request_id,
            request.method(),
            request.uri(),
        ));

        let url = self.resolve_url(request.uri());

        if url.is_none() {
            return self.notfound(BoxBody::new(request));
        }

        // TODO use everything from original request (method, body, ...)
        let result: Response<BoxBody<Bytes, hyper::Error>> = proxy_response(url.unwrap()).await?;

        self.logger.info(&format!(
            "Request \"{}\" finished - took {}ms",
            request_id,
            start.elapsed().as_millis()
        ));
        Ok(result)
    }
}

pub type ErrorType = Box<dyn std::error::Error + Send + Sync>;
