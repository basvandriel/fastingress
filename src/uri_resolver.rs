use hyper::Uri;

use crate::{route_entry::RouteEntry, service_resolver::resolve_service_uri};

pub trait UrlResolver {
    fn resolve(&self, loc: &RouteEntry) -> Option<Uri>;
}

pub struct ProxiedServiceURLResolver {
    pub original_url: Uri,
}
impl ProxiedServiceURLResolver {
    fn resolve_non_rootpath(&self, ingress_route: &str, request_path: &str) -> &str {
        // This handles the case where the incoming route
        // is equal to te defined path on the matched ingress route
        if ingress_route == request_path {
            return "/";
        }

        let stripped = request_path.strip_prefix(ingress_route).unwrap();

        "/"
    }
}
impl UrlResolver for ProxiedServiceURLResolver {
    fn resolve(&self, loc: &RouteEntry) -> Option<Uri> {
        let mut service_url = resolve_service_uri(loc).to_string();

        service_url += &format!(":{}", loc.port);

        // For some reason, the url always have to end
        // with a slash.
        service_url += "/proxy";

        let pathquery = self.original_url.path_and_query().expect("Should have");

        // TODO this has an issue. When a service is listening on "/example/" for example, this shouldn't
        // be added to the URL

        // do nothing, only add query

        let incomingpath = pathquery.path();

        if incomingpath != "/" {
            service_url += self.resolve_non_rootpath(&loc.route, incomingpath);
        } else {
            service_url += "/"
        }
        // service_url += &pathquery.to_string();

        // if let Some(path_and_query) = self.original_url.path_and_query() {
        // }
        Some(service_url.parse::<Uri>().unwrap())
    }
}

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
