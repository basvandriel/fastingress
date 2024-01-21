use hyper::Uri;

use crate::service_resolver::{resolve_service_info, KubeServiceLocation};

pub struct IpFinder {}

impl IpFinder {
    pub fn find(&self, service_location: &KubeServiceLocation) -> Option<Uri> {
        let service_url = resolve_service_info(service_location);

        let _ = service_location;
        // TODO find the URI of a service in a kubernetes cluster
        return None;
    }
}
