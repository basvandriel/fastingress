use fastingress::service_resolver::{build_service_proxy_url, KubeServiceLocation};
use hyper::Uri;

#[test]
fn it_finds_correct_service() {
    let loc = KubeServiceLocation {
        namespace: String::from("default"),
        name: String::from("nginx-service"),
        port: 80,
    };

    let currentip = "http://localhost:3000/ip".parse::<Uri>().unwrap();

    let result = build_service_proxy_url(&loc, &currentip);

    assert_eq!(
        result,
        "http://localhost:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/ip"
    );
}
