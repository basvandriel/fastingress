use std::sync::{Arc, Mutex};

use k8s_openapi::api::networking::v1::IngressRule;

use crate::route_entry::RouteEntry;

pub type Arced<T> = Arc<Mutex<T>>;
pub type RouteMap = Vec<IngressRule>;

pub type NewRouteMap = Vec<RouteEntry>;
