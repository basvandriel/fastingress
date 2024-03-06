use fastingress::{route_entry::RouteEntry, service_resolver::build_service_proxy_url};
use hyper::Uri;

#[test]
fn it_finds_correct_service() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip".parse::<Uri>().unwrap();

    let result = build_service_proxy_url(&loc, &currentip);

    assert_eq!(
        result,
        "http://localhost:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/ip"
    );
}
