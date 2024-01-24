use fastingress::{
    api_resolver::{resolve_in_cluster_service_uri, resolve_in_cluster_uri},
    service_resolver::KubeServiceLocation,
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

    let result = resolve_in_cluster_service_uri(&loc, &currentip).expect("Should work");
    let host = result.host().unwrap();
    assert_eq!(host, "nginx-service.default.svc.cluster.local");
}
