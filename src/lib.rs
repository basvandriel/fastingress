pub mod api_resolver;
pub mod constants;
pub mod ingress;
pub mod ipfinder;
pub mod kube_api_structs;
pub mod logger;
pub mod paths;
pub mod proxy;
pub mod service_resolver;
pub mod uri_resolver;

pub mod request_handler;
pub mod utils;

pub mod types;

pub mod ingress_resource_json_parser;

pub mod api_watcher;
pub mod ingress_resource_resolver;

// pub async fn hello(req: Request<Incoming>) -> Result<R, ErrorType> {
//     // return IngressRequestHandler.proxy_to_service(req).await;
// }

// pub async fn accept_incoming(stream: TcpStream, routes: RouteMap) {
//     let logger = Logger {};
//     logger.info(format!("Amount of routes: {:?}", routes.len()).as_str());

//     let io = TokioIo::new(stream);
//     let http = HTTPBuilder::new();

//     // http.serve_connection(io, service)
//     if let Err(err) = http.serve_connection(io, service_fn(hello)).await {
//         println!("Error serving connection: {:?}", err);
//     }
// }
