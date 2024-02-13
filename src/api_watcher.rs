use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::{HTTPIngressPath, Ingress, IngressRule, IngressSpec};

use crate::constants::INGRESS_CLASSNAME;
use crate::logger::Logger;
use crate::route_entry::RouteEntry;
use crate::types::{Arced, NewRouteMap};
use kube::{
    runtime::{watcher, WatchStreamExt},
    Api, Client,
};

pub struct APIListener {
    pub logger: Logger,
    pub routes: Arced<NewRouteMap>,
}

impl APIListener {
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
            };
            entries.push(x);
        }
        entries
    }

    fn resolve_route_entries(
        &self,
        routemap: &Vec<IngressRule>,
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

    async fn handle_ingress(&self, ingress: &IngressSpec, name: &String) {
        let rules: Vec<IngressRule> = ingress
            .to_owned()
            .rules
            .expect("Ingress Rules should be there");

        self.logger.info("Processing Ingress definition...");
        let mut entries = self.resolve_route_entries(&rules, name);

        let mut x = self.routes.lock().unwrap();
        x.append(&mut entries);
    }

    fn resolve_ingress_class<'a>(&'a self, ingress: &'a Ingress) -> &String {
        let cn = ingress
            .spec
            .as_ref()
            .expect("spec should be there")
            .ingress_class_name
            .as_ref()
            .expect("class name should be there");

        cn
    }

    pub async fn listen(&self) {
        let client = Client::try_default().await.expect("Kube client");
        let api = Api::<Ingress>::default_namespaced(client);

        let conf = watcher::Config::default();

        let stream = watcher(api, conf).applied_objects();
        pin_mut!(stream);

        let logmessage = format!(
            "Listening for new Kubernetes Ingress events with classname '{INGRESS_CLASSNAME}'"
        );
        self.logger.info(&logmessage);

        while let Some(ingress) = stream.try_next().await.unwrap() {
            let ingress_class = self.resolve_ingress_class(&ingress);
            let ingress_name = ingress.metadata.name.as_ref().unwrap();

            if ingress_class != INGRESS_CLASSNAME {
                continue;
            }
            self.handle_ingress(&ingress.spec.unwrap(), ingress_name)
                .await;
        }
    }
}
