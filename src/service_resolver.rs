use std::net::SocketAddr;

const DEFAULT_CLUSTER_IP: &str = "localhost";
const DEFAULT_DOCKER_DESKTOP_CLUSTER_IP: &str = "kubernetes.docker.internal";

const DEFAULT_CLUSTER_PORT: u16 = 6443;
const DEFAULT_PROXY_PORT: u16 = 8001;

pub fn resolve_service_ip(name: &str) -> SocketAddr {
    let protocol: &str = "http";
    // TODO find and define Kubernetes cluster ip + port
    // defaults to https://localhost:6443 or  https://kubernetes.docker.internal:6443 for docker desktop
    //
    // TODO find how to access the local kubernetes cluster IP's.
    // TODO authenticate to the Kubernetes API
    // TODO find service in Kubernetes API and retrieve cluster IP
    // TODO test IP connectivity in a later integration test
    println!("Resolving {name}");
    let ip = SocketAddr::from(([127, 0, 0, 1], 8000));

    return ip;
}
