use fastingress::{
    route_entry::RouteEntry,
    urlresolving::{proxied_url_resolver::ProxiedServiceURLResolver, UrlResolver},
};
use hyper::Uri;

#[test]
fn it_resolves_one_param() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip?message=hi"
        .parse::<Uri>()
        .unwrap();

    let resolver = ProxiedServiceURLResolver {
        original_url: currentip,
    };
    let actual = resolver.resolve(&loc);

    assert_eq!(actual.expect("Should be there").to_string(), "http://localhost:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/ip?message=hi")
}

#[test]
fn it_resolves_multi_params() {
    let loc = RouteEntry {
        namespace: String::from("default"),
        service: "nginx-service".to_string(),
        ingress_name: "nomatter".to_string(),
        port: 80,
        route: "/".to_owned(),
        host: "localhost".to_owned(),
    };

    let currentip = "http://localhost:3000/ip?message=hi&status=0&health=ok"
        .parse::<Uri>()
        .unwrap();

    let resolver = ProxiedServiceURLResolver {
        original_url: currentip,
    };
    let actual = resolver.resolve(&loc);

    assert_eq!(actual.expect("Should be there").to_string(), "http://localhost:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/ip?message=hi&status=0&health=ok")
}
