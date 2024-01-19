use std::net::SocketAddr;

use fastingress::service_resolver::{build_service_proxy_url, resolve_service_ip};

#[test]
fn it_adds_two() {
    let x: SocketAddr = resolve_service_ip("nginx-service");

    assert_eq!(x.port(), 8000);
    assert_eq!(x.ip().to_string(), "127.0.0.1");
    assert_eq!(x.is_ipv4(), true);
}

#[test]
fn it_finds_correct_service() {
    let result = build_service_proxy_url("nginx-service", 80);

    assert_eq!(
        result,
        "http://localhost:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/"
    );
}
