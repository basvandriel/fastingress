use fs::File;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, fs, io::BufReader, path::PathBuf};

fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn get_kubernetes_path() -> PathBuf {
    let mut base = get_project_root();
    base.push("kubernetes");

    return base;
}
#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRulePathBackendServicePort {
    number: u8,
}

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRulePathBackendService {
    name: String,
    port: KubeAPIObjectSpecRulePathBackendServicePort,
}

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRulePathBackend {
    service: KubeAPIObjectSpecRulePathBackendService,
}

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRulePath {
    path: String,
    #[serde(rename = "pathType")]
    path_type: String,
    backend: KubeAPIObjectSpecRulePathBackend,
}

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecHTTPRule {
    paths: Vec<KubeAPIObjectSpecRulePath>,
}
#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRule {
    http: KubeAPIObjectSpecHTTPRule,
}

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpec {
    #[serde(rename = "ingressClassName")]
    classname: String,
    rules: Vec<KubeAPIObjectSpecRule>,
}

#[test]
fn it_shouldwork_strictly() {
    let mut jsonpath = get_kubernetes_path();
    jsonpath.push("sample_ingress_api_response.json");

    let file = File::open(jsonpath).expect("file should open");
    let reader = BufReader::new(file);

    #[derive(Serialize, Deserialize)]
    struct KubeAPIObject {
        // ingress_type: String,
        kind: String,

        #[serde(rename = "apiVersion")]
        api_version: String,
        spec: KubeAPIObjectSpec,
    }

    #[derive(Serialize, Deserialize)]
    struct Foo {
        object: KubeAPIObject,
    }
    let u: Foo = serde_json::from_reader(reader).expect("Should parse");

    assert_eq!(u.object.spec.rules[0].http.paths[0].path, "/");
    assert_eq!(u.object.spec.rules[0].http.paths[0].path_type, "Prefix");
    assert_eq!(
        u.object.spec.rules[0].http.paths[0].backend.service.name,
        "nginx-service"
    );
    assert_eq!(
        u.object.spec.rules[0].http.paths[0]
            .backend
            .service
            .port
            .number,
        80
    );
}

#[test]
fn it_shouldwork_justpath() {
    let mut jsonpath = get_kubernetes_path();
    jsonpath.push("sample_ingress_api_response.json");

    let file = File::open(jsonpath).expect("file should open");
    let reader = BufReader::new(file);

    let root: Value = serde_json::from_reader(reader).expect("Should parse");
    let entries = &root["object"].as_object().unwrap()["spec"];

    let spec: KubeAPIObjectSpec =
        serde_json::from_value(entries.to_owned()).expect("JSON should parse");

    assert_eq!(spec.rules[0].http.paths[0].path, "/");
}
