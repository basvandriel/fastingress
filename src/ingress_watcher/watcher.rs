use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::Ingress;

use crate::ingress_watcher::event_handler::IngressEventHandler;
use crate::ingress_watcher::ingress_appied_handler::IngressAppliedHandler;
use crate::logger::Logger;
use crate::route_entry::RouteEntry;
use crate::types::Arced;
use crate::{
    constants::INGRESS_CLASSNAME, ingress_watcher::ingress_delete_handler::IngressDeleteHandler,
};

use kube::runtime::watcher::Event::{Applied, Deleted, Restarted};
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

        let mut apply_handler = IngressAppliedHandler::new(routeclone);

        match event {
            Restarted(ingresses) => {
                for i in ingresses {
                    apply_handler.handle(&i);
                }
            }
            Applied(ingress) => {
                apply_handler.handle(&ingress);
            }
            Deleted(ingress) => {
                IngressDeleteHandler::new(self.routes.clone()).handle(&ingress);
            }
        };
    }

    pub async fn listen(self, kubeclient: Client) {
        let api = Api::<Ingress>::namespaced(kubeclient.to_owned(), "default");
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
