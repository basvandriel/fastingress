use http_body_util::Empty;
use hyper::{body::Bytes, Request, Uri};

use crate::utils::handshake_url;

// use http_body_util::Empty;
use http_body_util::BodyExt;
use tokio::io::{self, AsyncWriteExt as _};

pub struct LocalAPIIngressResourceResolver {}

impl LocalAPIIngressResourceResolver {
    fn get_uri(&self) -> Uri {
        let mut url = "http://localhost:8001/".to_string();
        url += "apis/networking.k8s.io/v1/";
        url += "ingresses?watch=true";

        return url.parse::<Uri>().expect("Parsing should work");
    }

    pub async fn resolve(&self) {
        let uri = self.get_uri();
        let authority = uri.authority().unwrap().clone();

        let req = Request::get(&uri)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())
            .unwrap();

        let mut sender = handshake_url::<Empty<Bytes>>(&uri).await;

        // Request::get(uri);
        let mut res = sender.send_request(req).await.expect("Should work");

        while let Some(next) = res.frame().await {
            let frame = next.unwrap();
            if let Some(chunk) = frame.data_ref() {
                io::stdout().write_all(chunk).await.unwrap();
            }
        }
    }
}
