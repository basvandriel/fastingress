use crate::route_entry::{self, RouteEntry};

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
        let result = fullpath.split("/").collect::<Vec<&str>>();
        result
    }

    fn match_parts_to_route(
        &self,
        incoming_parts: Vec<&str>,
        possible_match: &RouteEntry,
    ) -> Option<&RouteEntry> {
        let matching_parts = self.explode_webroute(&possible_match.route);
        let no_match_parts = matching_parts.len();
        let no_incoming_parts = incoming_parts.len();

        // For example, route entry /example/hi/bas
        // should never match to /example/hi
        if matching_parts.len() > incoming_parts.len() {
            return None;
        }

        // From here, only routes match that are
        // - The same amount in paths (incoming: "/example/hi", "/example/bas")
        // - Have a higher amount of paths (incoming: "/example/hi/bas"  route: "/example/hi")
        if matching_parts.len() == incoming_parts.len() {
            let actual_matches = matching_parts
                .iter()
                .zip(&incoming_parts)
                .filter(|&(a, b)| a == b)
                .count();

            let matching: bool = actual_matches == no_incoming_parts;

            if !matching {
                return None;
            }
            return None;
            // return Some(possible_match);
        } else {
            // If the incoming route has more parts then the route entry
            // that can be ok IF the route entry part works
        }

        None
    }
}

impl RouteMatcher for PrefixRouteMatcher {
    fn find(&self, path: &str) -> Option<&RouteEntry> {
        if self.routes.is_empty() {
            return None;
        }
        let parts = self.explode_webroute(path);
        // Explode the full path in parts so we can start comparing one by one
        // let parts = path.split("/").collect::<Vec<&str>>();

        for r in &self.routes {
            let x = self.match_parts_to_route(parts.clone(), r);
        }

        None
    }
}
