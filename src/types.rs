use std::sync::{Arc, Mutex};

use k8s_openapi::api::networking::v1::IngressRule;

pub type Arced<T> = Arc<Mutex<T>>;
pub type RouteMap = Vec<IngressRule>;
