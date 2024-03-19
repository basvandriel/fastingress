use k8s_openapi::api::networking::v1::Ingress;

pub trait IngressEventHandler {
    fn handle(&mut self, ingress: &Ingress);
}
