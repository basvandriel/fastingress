use crate::service_resolver::KubeServiceLocation;

#[derive(Clone, Debug)]
pub struct RouteEntry {
    pub ingress_name: String,
    pub host: String,
    pub route: String,
    pub service: String,
    pub port: i32,
    pub namespace: String,
}

impl RouteEntry {
    pub fn to_kube_servicelocation(&self) -> KubeServiceLocation {
        KubeServiceLocation {
            name: self.service.clone(),
            namespace: self.namespace.clone(),
            port: self.port,
        }
    }
}
