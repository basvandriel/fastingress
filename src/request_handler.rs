use crate::ingress::{ErrorType, IngressRequestHandler};
use crate::logger::Logger;
use crate::proxy::R;
use crate::types::{Arced, RouteMap};
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct Svc {
    pub logger: Logger,
    pub routes: Arced<RouteMap>,
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = R;
    type Error = ErrorType;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        let routes = self.routes.lock().unwrap().to_vec();

        let response = IngressRequestHandler.proxy_to_service(req, routes);
        Box::pin(response)
    }
}
