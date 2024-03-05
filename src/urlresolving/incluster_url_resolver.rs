use hyper::Uri;

use crate::route_entry::RouteEntry;

use super::UrlResolver;

pub struct InClusterServiceURLResolver {
    pub original_url: Uri,
}
impl UrlResolver for InClusterServiceURLResolver {
    fn resolve(&self, loc: &RouteEntry) -> Option<Uri> {
        let mut uri = format!("http://{}.{}.svc.", loc.service, loc.namespace);
        uri += Self::DEFAULT_CLUSTER_DNS_SUFFIX;
        uri += "/";

        if let Some(path_and_query) = self.original_url.path_and_query() {
            uri += &path_and_query.to_string();
        }
        Some(uri.parse::<Uri>().expect("Should parse service"))
    }
}
impl InClusterServiceURLResolver {
    const DEFAULT_CLUSTER_DNS_SUFFIX: &'static str = "cluster.local";
}
