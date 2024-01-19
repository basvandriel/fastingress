use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use crate::logger::log_request;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::service_resolver::build_service_proxy_url;
use crate::service_resolver::KubeServiceLocation;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler;

impl IngressRequestHandler {
    async fn proxy_to_service(request: Request<Incoming>) -> Result<R, ErrorType> {
        let start = Instant::now();

        let loc = KubeServiceLocation {
            namespace: String::from("default"),
            name: String::from("nginx-service"),
            port: 80,
        };
        let url = build_service_proxy_url(loc, request.uri());
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
