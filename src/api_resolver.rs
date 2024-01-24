use hyper::Uri;
use std::env;

use crate::service_resolver::{running_in_kubernetes_cluster, KubeServiceLocation};

const DEFAULT_CLUSTER_DNS_SUFFIX: &str = "cluster.local";

pub fn resolve_in_cluster_uri() -> Option<Uri> {
    if !running_in_kubernetes_cluster() {
        panic!("Should be running in Kubernetes cluster")
    }
    let base: String = env::var("KUBERNETES_SERVICE_HOST").unwrap();
    let uri = format!("https://{}/api", base).parse::<Uri>();

    return Some(uri.expect("should work here"));
}

pub fn resolve_in_cluster_service_uri(
    service_location: &KubeServiceLocation,
    original_uri: &Uri,
) -> Option<Uri> {
    let mut uri = format!(
        "http://{}.{}.svc.",
        service_location.name, service_location.namespace
    );
    uri += DEFAULT_CLUSTER_DNS_SUFFIX;
    uri += "/";

    if let Some(path_and_query) = original_uri.path_and_query() {
        uri += &path_and_query.to_string();
    }
    return Some(uri.parse::<Uri>().expect("Should parse service"));
}
