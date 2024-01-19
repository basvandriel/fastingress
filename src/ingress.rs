use hyper::body::Incoming;
use hyper::service::Service;
use hyper::Request;
use hyper::Uri;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

use crate::logger::log_request;
use crate::proxy::proxy_response;
use crate::proxy::R;
use crate::service_resolver::build_service_proxy_url;

#[derive(Debug, Clone)]
pub struct IngressRequestHandler;

impl IngressRequestHandler {
    pub async fn proxy_to_service(request: Request<Incoming>) -> Result<R, ErrorType> {
        let start = Instant::now();

        let url = build_service_proxy_url("nginx-service", 80).parse::<Uri>()?;
        let result = proxy_response(url).await?;

        log_request(request, start.elapsed().as_millis());
        Ok(result)
    }
}

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

impl Service<Request<Incoming>> for IngressRequestHandler {
    type Response = R;
    type Error = ErrorType;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let res = Self::proxy_to_service(req);
        Box::pin(res)
    }
}
