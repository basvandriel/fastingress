use std::net::SocketAddr;

use fastingress::service_resolver::resolve_service_ip;

#[test]
fn it_adds_two() {
    let x: SocketAddr = resolve_service_ip("nginx-service");

    assert_eq!(x.port(), 8000);
    assert_eq!(x.ip().to_string(), "127.0.0.1");
    assert_eq!(x.is_ipv4(), true);
}
