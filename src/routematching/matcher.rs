use crate::route_entry::RouteEntry;

pub trait RouteMatcher {
    fn find(&self, path: &str) -> Option<&RouteEntry>;
}

pub struct StrictRouteMatcher {
    pub existing_routes: Vec<RouteEntry>,
}
impl RouteMatcher for StrictRouteMatcher {
    fn find(&self, path: &str) -> Option<&RouteEntry> {
        self.existing_routes.iter().find(|r| r.route == path)
    }
}
