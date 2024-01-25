use hyper::Uri;
use std::env;

use crate::service_resolver::running_in_kubernetes_cluster;

pub fn resolve_in_cluster_uri() -> Option<Uri> {
    if !running_in_kubernetes_cluster() {
        panic!("Should be running in Kubernetes cluster")
    }
    let base: String = env::var("KUBERNETES_SERVICE_HOST").unwrap();
    let uri = format!("https://{}/api", base).parse::<Uri>();

    return Some(uri.expect("should work here"));
}
