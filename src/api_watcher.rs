use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::{Ingress, IngressRule, IngressSpec};

use crate::logger::Logger;
use crate::types::Arced;
use crate::{constants::INGRESS_CLASSNAME, types::RouteMap};
use kube::{
    runtime::{watcher, WatchStreamExt},
    Api, Client,
};

pub struct APIListener {
    pub logger: Logger,
    pub routes: Arced<RouteMap>,
}

impl APIListener {
    async fn handle_ingress(&self, ingress: &IngressSpec) {
        let rules: Vec<IngressRule> = ingress
            .to_owned()
            .rules
            .expect("Ingress Rules should be there");
        self.logger.info("Processing Ingress definition...");

        for rule in rules.iter() {
            let mut x = self.routes.lock().unwrap();
            x.push(rule.clone());
        }
    }

    fn resolve_ingress_class<'a>(&'a self, ingress: &'a Ingress) -> &String {
        let cn = ingress
            .spec
            .as_ref()
            .expect("spec should be there")
            .ingress_class_name
            .as_ref()
            .expect("class name should be there");

        return cn;
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
            if self.resolve_ingress_class(&ingress) != INGRESS_CLASSNAME {
                continue;
            }
            self.handle_ingress(&ingress.spec.unwrap()).await;
        }
    }
}
