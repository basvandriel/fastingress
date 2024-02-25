#[derive(Clone, Debug)]
pub struct RouteEntry {
    pub ingress_name: String,
    pub host: String,
    pub route: String,
    pub service: String,
    pub port: i32,
}
