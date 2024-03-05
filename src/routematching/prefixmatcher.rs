use crate::route_entry::RouteEntry;

use super::matcher::RouteMatcher;

// this module should cover:
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi/bas
//
// MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi/bas/hi2
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/hi__shouldnotmatch
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/example/bas
//
// NO MATCH
// route entry:     localhost:3000/example/hi
// incoming route:  localhost:3000/bas

pub struct PrefixRouteMatcher {
    routes: Vec<RouteEntry>,
}

impl PrefixRouteMatcher {
    pub fn new(routes: Vec<RouteEntry>) -> Self {
        Self { routes }
    }

    fn explode_webroute<'a>(&self, fullpath: &'a str) -> Vec<&'a str> {
        // TODO write something that will remove the trailing "/"
        let result = fullpath.split('/').collect::<Vec<&str>>();
        result
    }

    fn matches_route(&self, incoming_parts: &[&str], possible_match: &RouteEntry) -> bool {
        let matching_parts = self.explode_webroute(&possible_match.route);

        let no_match_parts = matching_parts.len();
        let no_incoming_parts = incoming_parts.len();

        // For example, route entry /example/hi/bas
        // should never match to /example/hi
        if no_match_parts > no_incoming_parts {
            return false;
        }

        // From here, only routes match that are
        // - The same amount in paths (incoming: "/example/hi", "/example/bas")
        // - Have a higher amount of paths (incoming: "/example/hi/bas"  route: "/example/hi")
        if no_match_parts == no_incoming_parts {
            return matching_parts.iter().eq(incoming_parts.iter());
        }
        // Take the amount of incoming parts that match the
        // number of expected parts.: std::iter::Take<std::slice::Iter<'_, &str>>
        let index_matched_incoming_parts = incoming_parts.iter().take(no_match_parts);

        let matches = matching_parts
            .iter()
            .zip(index_matched_incoming_parts)
            .filter(|&(a, b)| a == b)
            .count();

        matches == no_incoming_parts
    }
}

impl RouteMatcher for PrefixRouteMatcher {
    fn find(&self, path: &str) -> Option<&RouteEntry> {
        if self.routes.is_empty() {
            return None;
        }
        // Explode the full path in parts so we can start comparing one by one
        let parts = self.explode_webroute(path);

        self.routes.iter().find(|r| self.matches_route(&parts, r))
    }
}
