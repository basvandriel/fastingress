use crate::{constants::DEFAULT_PROXY_PORT, logger::Logger, utils::handshake_url};
use http_body_util::Empty;
use hyper::body::Bytes;
use kube::Client;

pub struct KubeClientResolver {
    logger: Logger,
}

impl KubeClientResolver {
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    async fn verify_api_access(&self, kubeclient: &Client) {
        kubeclient.apiserver_version().await.unwrap();
    }

    async fn verify_proxy_access(&self) {
        // TODO Make the URL based on the url thats being proxied
        let url = format!("http://localhost:{}", DEFAULT_PROXY_PORT)
            .parse::<hyper::Uri>()
            .expect("URL should be parsed");

        // Will panic
        handshake_url::<Empty<Bytes>>(&url).await;
    }

    pub async fn resolve(&self) -> Option<Client> {
        self.logger.info("Connecting to Kubernetes cluster...");

        // the app can start without connecting which will lead into unexpected errors
        let kubeclient = Client::try_default().await.expect("Client should connect");

        self.verify_api_access(&kubeclient).await;
        self.verify_proxy_access().await;

        // TODO we need proxy or in-server connectivity
        // can we do this per a regular http request?
        self.logger.info("OK");

        Some(kubeclient)
    }
}
