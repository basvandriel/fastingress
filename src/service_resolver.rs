use std::net::SocketAddr;

pub fn resolve_service_ip(name: &str) -> SocketAddr {
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
