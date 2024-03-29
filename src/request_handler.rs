use crate::ingress::{ErrorType, IngressRequestHandler};
use crate::logger::Logger;
use crate::proxy::R;
use crate::route_entry::RouteEntry;
use crate::types::Arced;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct Svc {
    pub logger: Logger,
    pub routes_clone: Arced<Vec<RouteEntry>>,
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = R;
    type Error = ErrorType;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        let routes = self.routes_clone.lock().unwrap().to_vec();

        let response = async move {
            let handler = IngressRequestHandler::new(routes);
            handler.proxy_to_service(req).await
        };
        Box::pin(response)
    }
}
