use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecRulePathBackendServicePort {
    pub number: u8,
}

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecRulePathBackendService {
    pub name: String,
    pub port: KubeAPIObjectSpecRulePathBackendServicePort,
}

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecRulePathBackend {
    pub service: KubeAPIObjectSpecRulePathBackendService,
}

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecRulePath {
    pub path: String,
    #[serde(rename = "pathType")]
    pub path_type: String,
    pub backend: KubeAPIObjectSpecRulePathBackend,
}

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecHTTPRule {
    pub paths: Vec<KubeAPIObjectSpecRulePath>,
}

#[derive(Serialize, Deserialize)]
pub struct KubeAPIObjectSpecRule {
    pub http: KubeAPIObjectSpecHTTPRule,
}
