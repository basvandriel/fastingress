use k8s_openapi::api::networking::v1::{HTTPIngressPath, Ingress, IngressRule, IngressSpec};

use crate::{constants::INGRESS_CLASSNAME, route_entry::RouteEntry, types::Arced};

use super::event_handler::IngressEventHandler;

pub struct IngressAppliedHandler {
    routes: Arced<Vec<RouteEntry>>,
}

impl IngressAppliedHandler {
    pub fn new(routes: Arced<Vec<RouteEntry>>) -> Self {
        Self { routes }
    }
    fn resolve_rule_entries(&self, route: &IngressRule) -> Vec<RouteEntry> {
        let http = route.http.as_ref().unwrap();
        let paths: Vec<HTTPIngressPath> = http.paths.clone();

        let mut entries: Vec<RouteEntry> = vec![];

        for pathobj in paths.iter() {
            let service = pathobj.backend.service.as_ref().unwrap();
            let port = service.port.as_ref().unwrap().number.unwrap();
            let path = pathobj.path.as_ref().unwrap();

            let x = RouteEntry {
                host: "localhost".to_string(),
                port,
                route: path.to_owned(),
                service: service.name.to_owned(),

                // Will be set later
                ingress_name: String::new(),
                namespace: "default".to_string(),
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
        ingress_name: &String,
    ) -> Vec<RouteEntry> {
        routemap
            .iter()
            .flat_map(|route| self.resolve_rule_entries(route))
            .map(|mut inf| {
                inf.ingress_name = ingress_name.to_owned();
                inf
            })
            .collect()
    }

    fn resolve_ingress(&self, ingress: &IngressSpec, name: &String) -> Vec<RouteEntry> {
        let rules: Vec<IngressRule> = ingress
            .to_owned()
            .rules
            .expect("Ingress Rules should be there");

        self.resolve_route_entries(&rules, name)
    }

    fn resolve_ingress_class<'a>(&'a self, ingress: &'a Ingress) -> &String {
        ingress
            .spec
            .as_ref()
            .expect("spec should be there")
            .ingress_class_name
            .as_ref()
            .expect("class name should be there")
    }
}

impl IngressEventHandler for IngressAppliedHandler {
    fn handle(&mut self, ingress: &k8s_openapi::api::networking::v1::Ingress) {
        if self.resolve_ingress_class(&ingress) != INGRESS_CLASSNAME {
            return;
        }
        let ingress_name = ingress.metadata.name.as_ref().unwrap();
        let ingress_spec = ingress.spec.as_ref().unwrap();

        let routes = self.resolve_ingress(ingress_spec, ingress_name);

        let mut payload = self.routes.lock().unwrap();

        // If it is already routed, let's delete it first. It's easier then adding the routes one by one.
        // Meaning, everything that returns false based on this func will get deleted
        payload.retain(|r| &r.ingress_name != ingress_name);

        // See https://github.com/basvandriel/fastingress/issues/11
        payload.extend(routes);
    }
}
