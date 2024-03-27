use k8s_openapi::api::networking::v1::Ingress;

use crate::{
    constants::INGRESS_CLASSNAME, route_entry::RouteEntry, route_entry_resolver::IngressResolver,
    types::Arced,
};

use super::event_handler::IngressEventHandler;

pub struct IngressAppliedHandler {
    routes: Arced<Vec<RouteEntry>>,
}

impl IngressAppliedHandler {
    pub fn new(routes: Arced<Vec<RouteEntry>>) -> Self {
        Self { routes }
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

    fn convert_ingress_to_route_entries(
        &self,
        ingress: &k8s_openapi::api::networking::v1::Ingress,
        ingress_name: &str,
    ) -> Vec<RouteEntry> {
        let ingress_spec = ingress.spec.as_ref().unwrap();
        IngressResolver {}.to_route_entries(ingress_spec, ingress_name)
    }
}

impl IngressEventHandler for IngressAppliedHandler {
    fn handle(&mut self, ingress: &k8s_openapi::api::networking::v1::Ingress) {
        if self.resolve_ingress_class(ingress) != INGRESS_CLASSNAME {
            return;
        }
        let ingress_name = ingress.metadata.name.as_ref().unwrap();
        let route_entries = self.convert_ingress_to_route_entries(ingress, ingress_name);

        let mut payload = self.routes.lock().unwrap();

        // If it is already routed, let's delete it first. It's easier then adding the routes one by one.
        // Meaning, everything that returns false based on this func will get deleted
        payload.retain(|r| &r.ingress_name != ingress_name);

        // See https://github.com/basvandriel/fastingress/issues/11
        payload.extend(route_entries);
    }
}
