use crate::{route_entry::RouteEntry, types::Arced};

use super::event_handler::IngressEventHandler;

pub struct IngressDeleteHandler {
    routes: Arced<Vec<RouteEntry>>,
}

impl IngressDeleteHandler {
    pub fn new(routes: Arced<Vec<RouteEntry>>) -> Self {
        Self { routes }
    }
}

impl IngressEventHandler for IngressDeleteHandler {
    fn handle(&mut self, ingress: &k8s_openapi::api::networking::v1::Ingress) {
        let mut payload = self.routes.lock().unwrap();

        if payload.is_empty() {
            return;
        }
        let name_to_delete = ingress
            .metadata
            .name
            .as_ref()
            .expect("Ingress should have a name");

        if let Some(index) = payload
            .iter()
            .position(|r| &r.ingress_name == name_to_delete)
        {
            payload.remove(index);
        }
    }
}
