use hyper::Uri;

use crate::{route_entry::RouteEntry, service_resolver::resolve_service_uri};

use super::UrlResolver;

pub struct ProxiedServiceURLResolver {
    pub original_url: Uri,
}
impl ProxiedServiceURLResolver {
    fn resolve_non_rootpath(&self, ingress_route: &str, request_path: &str) -> String {
        let stripped = request_path.strip_prefix(ingress_route).unwrap();

        stripped.to_string()
    }
}
impl UrlResolver for ProxiedServiceURLResolver {
    fn resolve(&self, loc: &RouteEntry) -> Option<Uri> {
        let mut service_url = resolve_service_uri(loc).to_string();
        service_url += &format!(":{}", loc.port);

        // The url always have to end with a slash.
        service_url += "/proxy/";

        let pathquery = self.original_url.path_and_query().expect("Should have");
        let incomingpath = pathquery.path();

        let direct_match = loc.route == incomingpath;

        if incomingpath != "/" && !direct_match {
            service_url += &self.resolve_non_rootpath(&loc.route, incomingpath);
        }
        Some(service_url.parse::<Uri>().unwrap())
    }
}
