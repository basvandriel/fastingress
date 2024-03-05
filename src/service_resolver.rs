use std::env;

use hyper::Uri;

use crate::{
    constants::{DEFAULT_CLUSTER_IP, DEFAULT_PROXY_PORT},
    route_entry::RouteEntry,
};

pub fn running_in_kubernetes_cluster() -> bool {
    env::var("KUBERNETES_SERVICE_HOST").is_ok()
}

pub struct KubeServiceLocation {
    pub namespace: String,
    pub name: String,
    pub port: i32,
}

pub fn resolve_service_uri(service_loc: &RouteEntry) -> Uri {
    // TODO this does not work in the cluster. localhost will only be available
    // when we're proxying. We should get tmy-backend-service.default.svc.cluster.local
    // TODO also rename
    let mut url: String = format!("http://{}", DEFAULT_CLUSTER_IP);
    url += &format!(":{}", DEFAULT_PROXY_PORT);

    url += &format!("/api/v1/namespaces/{}/services/", service_loc.namespace);
    url += &service_loc.service;

    url.parse::<Uri>().unwrap()
}

pub fn build_service_proxy_url(service_loc: &RouteEntry, original_uri: &Uri) -> Uri {
    let mut service_url = resolve_service_uri(service_loc).to_string();

    service_url += &format!(":{}", service_loc.port);
    service_url += "/proxy";

    if let Some(path_and_query) = original_uri.path_and_query() {
        service_url += &path_and_query.to_string();
    }
    service_url.parse::<Uri>().unwrap()
}
