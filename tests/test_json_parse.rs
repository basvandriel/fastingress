use fastingress::{
    constants::INGRESS_CLASSNAME, kube_api_structs::KubeAPIObjectSpecRule,
    paths::get_kubernetes_path,
};
use fs::File;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, io::BufReader};

fn resolve_sample_file() -> File {
    let mut jsonpath = get_kubernetes_path();
    jsonpath.push("sample_ingress_api_response.json");

    File::open(jsonpath).expect("file should open")
}

#[test]
fn it_shouldwork_strictly() {
    let file: File = resolve_sample_file();
    let reader = BufReader::new(file);

    #[derive(Serialize, Deserialize)]
    pub struct KubeAPIObjectSpec {
        #[serde(rename = "ingressClassName")]
        pub classname: String,
        pub rules: Vec<KubeAPIObjectSpecRule>,
    }
    #[derive(Serialize, Deserialize)]
    struct KubeAPIObject {
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
fn it_shouldwork_rules() {
    let file: File = resolve_sample_file();
    let reader = BufReader::new(file);

    let root: Value = serde_json::from_reader(reader).expect("Should parse");
    let entries = &root["object"].as_object().unwrap()["spec"];

    assert_eq!(entries["ingressClassName"], INGRESS_CLASSNAME);

    let (_, rules) = entries.as_object().unwrap().get_key_value("rules").unwrap();

    let rulesobje: Vec<KubeAPIObjectSpecRule> =
        serde_json::from_value(rules.to_owned()).expect("should parse");

    assert_eq!(rulesobje[0].http.paths[0].path, "/")
}
