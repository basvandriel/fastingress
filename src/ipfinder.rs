use std::net::Ipv4Addr;

use crate::{
    service_resolver::{resolve_service_uri, KubeServiceLocation},
    utils::handshake_url,
};
use http_body_util::{BodyExt, Empty};
use hyper::{self, client::conn::http1::SendRequest};
use hyper::{
    body::{Bytes, Incoming},
    Request, Response,
};
use serde::Deserialize;

pub struct ProxiedServiceIPFinder {}

impl ProxiedServiceIPFinder {
    async fn request_json(service_location: &KubeServiceLocation) -> Response<Incoming> {
        type BodyType = Empty<Bytes>;

        let service_url = resolve_service_uri(service_location);
        let authority = service_url.authority().unwrap().clone();

        let req = Request::get(&service_url)
            .header(hyper::header::HOST, authority.as_str())
            .body(BodyType::new())
            .unwrap();

        let mut sender: SendRequest<Empty<Bytes>> = handshake_url::<BodyType>(&service_url).await;

        return sender.send_request(req).await.unwrap();
    }

    pub async fn find(&self, service_location: &KubeServiceLocation) -> Option<Ipv4Addr> {
        let response = Self::request_json(service_location).await;

        #[derive(Deserialize)]
        struct Spec {
            #[serde(rename = "clusterIP")]
            cluster_ip: Ipv4Addr,
        }
        #[derive(Deserialize)]
        struct Service {
            spec: Spec,
        }

        let response_bytes = response.collect().await.unwrap();
        let service: Service = serde_json::from_slice(&response_bytes.to_bytes()).unwrap();

        let clusterip = service.spec.cluster_ip;

        return Some(clusterip);
    }
}
