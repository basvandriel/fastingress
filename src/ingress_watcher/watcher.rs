use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::{HTTPIngressPath, Ingress, IngressRule, IngressSpec};

use crate::ingress_watcher::event_handler::IngressEventHandler;
use crate::ingress_watcher::ingress_appied_handler::IngressAppliedHandler;
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
    async fn process_ingress_event(&self, event: watcher::Event<Ingress>) {
        let routeclone = self.routes.clone();

        // TODO we can create an event handler and return that. Then call that
        match event {
            watcher::Event::Applied(ingress) => {
                IngressAppliedHandler::new(routeclone).handle(&ingress);
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
