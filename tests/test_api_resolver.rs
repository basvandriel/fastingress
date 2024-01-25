use fastingress::{
    api_resolver::resolve_in_cluster_uri,
    service_resolver::KubeServiceLocation,
    uri_resolver::{InClusterServiceURLResolver, UrlResolver},
};
use hyper::Uri;

#[test]
#[should_panic(expected = "Should be running in Kubernetes cluster")]
fn it_should_panic() {
    resolve_in_cluster_uri();
}

#[test]
fn it_should_generate_one() {
    let loc = KubeServiceLocation {
        namespace: String::from("default"),
        name: String::from("nginx-service"),
        port: 80,
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
