// use futures::{pin_mut, TryStreamExt};
use futures::{pin_mut, TryStreamExt};
use k8s_openapi::api::networking::v1::Ingress;

use kube::{
    runtime::{watcher, WatchStreamExt},
    Api, Client,
};

#[tokio::test]
async fn it_should_wo() {
    // let x = LocalAPIIngressResourceResolver {};
    // let z = x.resolve().await;
    let client = Client::try_default().await.expect("Kube client");

    let api = Api::<Ingress>::default_namespaced(client);

    let conf = watcher::Config::default();

    let stream = watcher(api, conf).applied_objects();
    pin_mut!(stream);

    while let Some(ingress) = stream.try_next().await.unwrap() {
        let spec = ingress.spec.expect("Spec should be there");
        println!("hi");
    }
}
