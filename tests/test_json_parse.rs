use fs::File;
use serde::{Deserialize, Serialize};
use std::{env, fs, io::BufReader, path::PathBuf};

fn get_project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn get_kubernetes_path() -> PathBuf {
    let mut base = get_project_root();
    base.push("kubernetes");

    base
}

s

#[derive(Serialize, Deserialize)]
struct KubeAPIObjectSpecRulePath {
    path: String,
    #[serde(rename = "pathType")]
    path_type: String,
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

#[test]
fn it_shouldwork() {
    let mut jsonpath = get_kubernetes_path();
    jsonpath.push("sample_ingress_api_response.json");

    let mut file = File::open(jsonpath).expect("file should open");

    let mut data = String::new();
    let reader = BufReader::new(file);

    let u: Foo = serde_json::from_reader(reader).expect("Should parse");

    println!("hi")
}
