use fastingress::{
    route_entry::RouteEntry,
    routematching::{matcher::RouteMatcher, prefixmatcher::PrefixRouteMatcher},
};

#[test]
fn it_should_return_none_when_empty() {
    let matcher = PrefixRouteMatcher::new(vec![]);
    let result = matcher.find("no");

    assert_eq!(result.is_none(), true)
}

#[test]
fn it_should_not_match_any_higher_amount_route_entries() {
    let route_entry_path = "/example/hi/bas";
    let incoming_path = "/example/hi";

    let longentry = RouteEntry {
        ingress_name: "sample".to_string(),
        host: "".to_string(),
        route: route_entry_path.to_owned(),
        service: "".to_string(),
        port: 80,
        namespace: "".to_string(),
    };
    let matcher = PrefixRouteMatcher::new(vec![longentry]);
    let result = matcher.find(incoming_path);

    assert_eq!(result.is_none(), true)
}

#[test]
fn it_should_fail_on_same_length_routes() {
    let route_entry_path = "/example/hi";
    let incoming_path = "/example/bas";

    let longentry = RouteEntry {
        ingress_name: "".to_string(),
        host: "".to_string(),
        service: "".to_string(),
        namespace: "".to_string(),
        port: 80,
        route: route_entry_path.to_owned(),
    };
    let matcher = PrefixRouteMatcher::new(vec![longentry]);
    let result = matcher.find(incoming_path);

    assert_eq!(result.is_none(), true)
}

#[test]
fn it_should_succeed_on_same_length_routes() {
    let route_entry_path = "/example/bas";
    let incoming_path = "/example/bas";

    let longentry = RouteEntry {
        ingress_name: "".to_string(),
        host: "".to_string(),
        service: "".to_string(),
        namespace: "".to_string(),
        port: 80,
        route: route_entry_path.to_owned(),
    };
    let matcher = PrefixRouteMatcher::new(vec![longentry]);
    let result = matcher.find(incoming_path);

    assert_eq!(result.is_none(), false)
}

#[test]
fn it_should_fail_on_extra_routes() {
    let route_entry_path = "/example/bas";
    let incoming_path = "/example/kaas/hi/bas";

    let longentry = RouteEntry {
        ingress_name: "".to_string(),
        host: "".to_string(),
        service: "".to_string(),
        namespace: "".to_string(),
        port: 80,
        route: route_entry_path.to_owned(),
    };
    let matcher = PrefixRouteMatcher::new(vec![longentry]);
    let result = matcher.find(incoming_path);

    assert_eq!(result.is_none(), true)
}

#[test]
fn it_should_succeed_on_extra_routes() {
    let route_entry_path = "/example/bas";
    let incoming_path = "/example/bas/hi/man";

    let longentry = RouteEntry {
        ingress_name: "".to_string(),
        host: "".to_string(),
        service: "".to_string(),
        namespace: "".to_string(),
        port: 80,
        route: route_entry_path.to_owned(),
    };
    let matcher = PrefixRouteMatcher::new(vec![longentry]);
    let result = matcher.find(incoming_path);

    assert_eq!(result.is_none(), true)
}
