use std::{env, net::SocketAddr};

use hyper::Uri;

use crate::constants::{DEFAULT_CLUSTER_IP, DEFAULT_PROXY_PORT};

fn running_in_kubernetes_cluster() -> bool {
    return env::var("KUBERNETES_SERVICE_HOST").is_ok();
}

fn should_proxy() -> bool {
    return running_in_kubernetes_cluster();
}

pub struct KubeServiceLocation {
    pub namespace: String,
    pub name: String,
    pub port: u16,
}

pub fn build_service_proxy_url(service_loc: KubeServiceLocation, original_uri: &Uri) -> Uri {
    let mut url: String = format!("http://{}", DEFAULT_CLUSTER_IP);
    url += &format!(":{}", DEFAULT_PROXY_PORT);

    url += &format!("/api/v1/namespaces/{}/services/", service_loc.namespace);
    url += &service_loc.name;

    url += &format!(":{}", service_loc.port);
    url += "/proxy";

    if let Some(path_and_query) = original_uri.path_and_query() {
        url += &path_and_query.to_string();
    }
    return url.parse::<Uri>().unwrap();
}

pub fn resolve_service_ip(name: &str) -> SocketAddr {
    if !should_proxy() {
        // TODO
    }
    println!("Resolving {name}");
    // let z = x +=
    // let api_url = "http://" + DEFAULT_CLUSTER_IP + ":";
    // TODO find and define Kubernetes cluster ip + port
    // defaults to https://localhost:6443 or  https://kubernetes.docker.internal:6443 for docker desktop
    //
    // TODO find service in Kubernetes API and retrieve cluster IP
    // TODO test IP connectivity in a later integration test
    let ip = SocketAddr::from(([127, 0, 0, 1], 8000));

    return ip;
}
