use std::net::SocketAddr;

use hyper::Uri;

use crate::constants::{DEFAULT_CLUSTER_IP, DEFAULT_PROXY_PORT};

fn should_proxy() -> bool {
    // For now, this value can always be true
    // since we're not testing kubernetes deployments yet
    true
}

pub struct KubeServiceLocation {
    pub namespace: String,
    pub name: String,
    pub port: u16,
}

pub fn build_service_proxy_url(service_loc: KubeServiceLocation) -> Uri {
    let mut url: String = format!("http://{}", DEFAULT_CLUSTER_IP);
    url += &format!(":{}", DEFAULT_PROXY_PORT);

    // TODO make the namespace configurable
    let namespace = "default";
    url += &format!("/api/v1/namespaces/{}/services/", namespace);
    url += service_loc.name.as_str();

    url += &format!(":{}", service_loc.port);

    // We need an ending slash
    // in order for it to redirect
    url += "/proxy/";

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
