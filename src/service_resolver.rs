use std::net::SocketAddr;

pub fn resolve_service_ip(name: &str) -> SocketAddr {
    println!("Resolving {name}");
    let ip = SocketAddr::from(([127, 0, 0, 1], 8000));

    return ip;
}
