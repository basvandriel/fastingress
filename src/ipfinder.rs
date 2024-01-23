use crate::{
    service_resolver::{resolve_service_uri, KubeServiceLocation},
    utils::handshake_url,
};
use http_body_util::{BodyExt, Empty};
use hyper::{self, body::Buf};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response, Uri,
};
use serde::Deserialize;

pub struct IpFinder {}

impl IpFinder {
    async fn request_json(service_location: &KubeServiceLocation) -> Response<Incoming> {
        type BodyType = Empty<Bytes>;

        let service_url = resolve_service_uri(service_location);
        let authority = service_url.authority().unwrap().clone();

        // Create an HTTP request with an empty body and a HOST header
        let req = Request::builder()
            .uri(&service_url)
            .header(hyper::header::HOST, authority.as_str())
            .body(BodyType::new())
            .unwrap();

        let mut sender = handshake_url::<BodyType>(&service_url).await;

        return sender.send_request(req).await.unwrap();
    }

    pub async fn find(&self, service_location: &KubeServiceLocation) -> Option<Uri> {
        let response = Self::request_json(service_location).await;

        #[derive(Deserialize)]
        struct Spec {
            #[serde(rename = "clusterIP")]
            cluster_ip: String,
        }

        #[derive(Deserialize)]
        struct Service {
            spec: Spec,
        }
        let what = response.collect().await.unwrap().aggregate();
        let reader = what.reader();

        let service: Service = serde_json::from_reader(reader).unwrap();
        let clusterip = service.spec.cluster_ip;

        // TODO find the URI of a service in a kubernetes cluster (spec >> clusterIP)
        return None;
    }
}
