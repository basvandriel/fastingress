use fastingress::{
    ingress_resource_json_parser::parse_ingress_rules, kube_api_structs::KubeAPIObjectSpecRule,
    paths::get_kubernetes_path,
};
use fs::File;

use std::{borrow::Borrow, fs, io::Read};

fn resolve_sample_file() -> File {
    let mut jsonpath = get_kubernetes_path();
    jsonpath.push("sample_ingress_api_response.json");

    File::open(jsonpath).expect("file should open")
}

#[test]
#[should_panic]
fn it_should_panic() {
    let data = r#"
    {
        "object": {
            "kind": "Ingress",
            "spec": {
                "ingressClassName": "fast",
                "rules": []
            }
        }
    }
    "#;
    parse_ingress_rules(&data);
}

#[test]
fn it_shouldwork_strictly() {
    let mut file: File = resolve_sample_file();
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).expect("Should parse");

    let rules: Vec<KubeAPIObjectSpecRule> = parse_ingress_rules(&buffer);
    let path = rules[0].http.paths[0].borrow();

    assert_eq!(path.path, "/");
    assert_eq!(path.path_type, "Prefix");
    assert_eq!(path.backend.service.name, "nginx-service");
    assert_eq!(path.backend.service.port.number, 80);
}
