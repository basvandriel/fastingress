use fastingress::{
    api_resolver::resolve_in_cluster_uri,
    route_entry::RouteEntry,
    urlresolving::{incluster_url_resolver::InClusterServiceURLResolver, UrlResolver},
};
use hyper::Uri;

#[test]
#[should_panic(expected = "Should be running in Kubernetes cluster")]
fn it_should_panic() {
    resolve_in_cluster_uri();
}

#[test]
fn it_should_generate_one() {
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
        original_url: currentip.clone(),
    };
    let result = resolver.resolve(&loc).expect("Should work");
    assert_eq!(
        result.host().unwrap(),
        "nginx-service.default.svc.cluster.local"
    );
}
