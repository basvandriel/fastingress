use fastingress::{
    route_entry::RouteEntry,
    urlresolving::{incluster_url_resolver::InClusterServiceURLResolver, UrlResolver},
};
use hyper::Uri;

#[test]
fn it_finds_url_nonroot() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip".parse::<Uri>().unwrap();

    let resolver = InClusterServiceURLResolver {
        original_url: currentip,
    };
    let actual = resolver.resolve(&loc).expect("URI should be there");
    let stringified = actual.to_string();

    assert_eq!(
        stringified,
        "http://nginx-service.default.svc.cluster.local/ip"
    );
}

#[test]
fn it_finds_url_with_one_query_param() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip?message=hi"
        .parse::<Uri>()
        .unwrap();

    let resolver = InClusterServiceURLResolver {
        original_url: currentip,
    };
    let actual = resolver.resolve(&loc).expect("URI should be there");
    let stringified = actual.to_string();

    assert_eq!(
        stringified,
        "http://nginx-service.default.svc.cluster.local/ip?message=hi"
    );
}

#[test]
fn it_finds_url_with_multi_query_params() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip?message=hi&status=0"
        .parse::<Uri>()
        .unwrap();

    let resolver = InClusterServiceURLResolver {
        original_url: currentip,
    };
    let actual = resolver.resolve(&loc).expect("URI should be there");
    let stringified = actual.to_string();

    assert_eq!(
        stringified,
        "http://nginx-service.default.svc.cluster.local/ip?message=hi&status=0"
    );
}
