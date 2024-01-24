use fastingress::{ipfinder, service_resolver::KubeServiceLocation};

#[tokio::test]
async fn it_should_find() {
    let loc: KubeServiceLocation = KubeServiceLocation {
        namespace: String::from("default"),
        name: String::from("nginx-service"),
        port: 80,
    };
    let handler = ipfinder::ProxiedServiceIPFinder {};
    let result = handler.find(&loc).await;

    assert!(result.is_some());
}
