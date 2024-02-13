use std::sync::{Arc, Mutex};

use crate::route_entry::RouteEntry;

pub type Arced<T> = Arc<Mutex<T>>;
pub type NewRouteMap = Vec<RouteEntry>;
