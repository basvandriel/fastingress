use std::vec;

use serde_json::Value;

use crate::{constants::INGRESS_CLASSNAME, kube_api_structs::KubeAPIObjectSpecRule};

pub fn parse_ingress_rules(json: &str) -> Vec<KubeAPIObjectSpecRule> {
    let root: Value = serde_json::from_str(&json).expect("Should parse");
    let entries: &Value = &root["object"].as_object().unwrap()["spec"];

    if entries["ingressClassName"] != INGRESS_CLASSNAME {
        panic!("no")
    }
    let (_, rules) = entries.as_object().unwrap().get_key_value("rules").unwrap();

    serde_json::from_value(rules.to_owned()).expect("should parse")
}
