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

impl<'a> IngressEventHandler for IngressDeleteHandler {
    fn handle(&mut self, ingress: &k8s_openapi::api::networking::v1::Ingress) {
        let mut payload = self.routes.lock().unwrap();

        if payload.is_empty() {
            return;
        }
        println!("Event type: Deleted");

        payload.remove(0);
    }
}
