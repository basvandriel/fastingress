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
