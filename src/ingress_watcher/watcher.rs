use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::{HTTPIngressPath, Ingress, IngressRule, IngressSpec};

use crate::ingress_watcher::event_handler::IngressEventHandler;
use crate::logger::Logger;
use crate::route_entry::RouteEntry;
use crate::types::Arced;
use crate::{
    constants::INGRESS_CLASSNAME, ingress_watcher::ingress_delete_handler::IngressDeleteHandler,
};

use kube::{runtime::watcher, Api, Client};

pub struct APIListener {
    pub logger: Logger,

    // This is the list that will be updated, deleted and modified
    // based on incoming routes
    pub routes: Arced<Vec<RouteEntry>>,
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
                namespace: "default".to_string(),
            };

            self.logger.info(&format!(
                "Routing \"{}\" to \"{}\" on port {}",
                &x.route, &x.service, &x.port
            ));
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

    async fn resolve_ingress(&self, ingress: &IngressSpec, name: &String) -> Vec<RouteEntry> {
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

    async fn handle_ingress_update(&self, ingress: &Ingress) {
        if self.resolve_ingress_class(&ingress) != INGRESS_CLASSNAME {
            return;
        }
        // TODO holds a uid. Might be nice for finding?
        let ingress_name = ingress.metadata.name.as_ref().unwrap();

        let routes = self
            .resolve_ingress(ingress.spec.as_ref().unwrap(), ingress_name)
            .await;

        let mut payload = self.routes.lock().unwrap();

        // TODO this should not just extend.
        // See https://github.com/basvandriel/fastingress/issues/11
        payload.extend(routes);
    }

    async fn process_ingress_event(&self, event: watcher::Event<Ingress>) {
        let routeclone = self.routes.clone();

        // TODO we can create an event handler and return that. Then call that
        match event {
            watcher::Event::Applied(ingress) => {
                self.handle_ingress_update(&ingress).await;
            }
            watcher::Event::Deleted(ingress) => {
                IngressDeleteHandler::new(routeclone).handle(&ingress);
            }
            watcher::Event::Restarted(_ingress) => {
                // TODO check if there, if not add it
            }
        };

        println!("hi");
    }

    pub async fn listen(self) {
        let client = Client::try_default().await.expect("Kube client");
        let api = Api::<Ingress>::namespaced(client, "default");
        let conf = watcher::Config::default();

        let w = watcher(api, conf);
        pin_mut!(w);

        self.logger.info(&format!(
            "Listening for new Kubernetes Ingress events with classname '{INGRESS_CLASSNAME}'"
        ));

        while let Some(event) = w.try_next().await.unwrap() {
            self.process_ingress_event(event).await
        }
    }
}
