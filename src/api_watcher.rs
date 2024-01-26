use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::{Ingress, IngressSpec};

use kube::{
    runtime::{watcher, WatchStreamExt},
    Api, Client,
};

use crate::logger::Logger;

pub struct APIListener {}

impl APIListener {
    pub async fn listen(&self) {
        let logger = Logger {};

        let client = Client::try_default().await.expect("Kube client");
        let api = Api::<Ingress>::default_namespaced(client);

        let conf = watcher::Config::default();

        let stream = watcher(api, conf).applied_objects();
        pin_mut!(stream);

        logger.info("Listening new Kubernetes Ingress events...");

        while let Some(ingress) = stream.try_next().await.unwrap() {
            let _: IngressSpec = ingress.spec.expect("Spec should be there");
            logger.info("Message found!");
        }
    }
}
