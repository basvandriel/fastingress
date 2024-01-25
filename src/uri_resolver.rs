use hyper::Uri;

use crate::service_resolver::{resolve_service_uri, KubeServiceLocation};

pub trait UrlResolver {
    fn resolve(&self, loc: &KubeServiceLocation) -> Option<Uri>;
}

pub struct ProxiedServiceURLResolver {
    pub original_url: Uri,
}
impl UrlResolver for ProxiedServiceURLResolver {
    fn resolve(&self, loc: &KubeServiceLocation) -> Option<Uri> {
        let mut service_url = resolve_service_uri(loc).to_string();

        service_url += &format!(":{}", loc.port);
        service_url += "/proxy";

        if let Some(path_and_query) = self.original_url.path_and_query() {
            service_url += &path_and_query.to_string();
        }
        return Some(service_url.parse::<Uri>().unwrap());
    }
}

pub struct InClusterServiceURLResolver {
    pub original_url: Uri,
}
impl UrlResolver for InClusterServiceURLResolver {
    fn resolve(&self, loc: &KubeServiceLocation) -> Option<Uri> {
        let mut uri = format!("http://{}.{}.svc.", loc.name, loc.namespace);
        uri += Self::DEFAULT_CLUSTER_DNS_SUFFIX;
        uri += "/";

        if let Some(path_and_query) = self.original_url.path_and_query() {
            uri += &path_and_query.to_string();
        }
        return Some(uri.parse::<Uri>().expect("Should parse service"));
    }
}
impl InClusterServiceURLResolver {
    const DEFAULT_CLUSTER_DNS_SUFFIX: &'static str = "cluster.local";
}
