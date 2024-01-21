use fastingress::{ipfinder, service_resolver::KubeServiceLocation};

#[test]
fn it_should_find() {
    let loc: KubeServiceLocation = KubeServiceLocation {
        namespace: String::from("default"),
        name: String::from("nginx-service"),
        port: 80,
    };
    let handler = ipfinder::IpFinder {};
    let result = handler.find(&loc);

    assert!(result.is_none());
}
