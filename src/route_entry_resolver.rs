use k8s_openapi::api::networking::v1::{HTTPIngressPath, IngressRule, IngressSpec};

use crate::route_entry::RouteEntry;

pub struct IngressResolver;

impl IngressResolver {
    fn resolve_rule_entries(&self, route: &IngressRule, ingress_name: &str) -> Vec<RouteEntry> {
        let http = route.http.as_ref().unwrap();
        let paths: Vec<HTTPIngressPath> = http.paths.clone();

        let mut entries: Vec<RouteEntry> = vec![];

        for pathobj in paths.iter() {
            let host = String::from("localhost");
            let namespace = String::from("default");

            let backend = pathobj.backend.service.as_ref().unwrap();
            let service = backend.name.to_owned();
            let port = backend.port.as_ref().unwrap().number.unwrap();

            let route = pathobj.path.as_ref().unwrap().to_string();

            let x = RouteEntry {
                host,
                port,
                route,
                service,
                namespace,

                // Needs ownership
                ingress_name: ingress_name.to_string(),
            };

            let logmsg = &format!(
                "Routing \"{}\" to \"{}\" on port {}",
                &x.route, &x.service, &x.port
            );
            println!("{}", logmsg);

            entries.push(x);
        }
        entries
    }

    fn resolve_route_entries(
        &self,
        routemap: &[IngressRule],
        ingress_name: &str,
    ) -> Vec<RouteEntry> {
        routemap
            .iter()
            .flat_map(|route| self.resolve_rule_entries(route, ingress_name))
            .collect()
    }

    pub fn to_route_entries(&self, ingress: &IngressSpec, name: &str) -> Vec<RouteEntry> {
        let rules = ingress
            .rules
            .as_ref()
            .expect("Ingress rules should be there");

        self.resolve_route_entries(rules, name)
    }
}
