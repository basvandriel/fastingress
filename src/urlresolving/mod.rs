use hyper::Uri;

use crate::route_entry::RouteEntry;

pub trait UrlResolver {
    fn resolve(&self, loc: &RouteEntry) -> Option<Uri>;
}

pub mod incluster_url_resolver;
pub mod proxied_url_resolver;
