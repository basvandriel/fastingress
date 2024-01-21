# Testing

First, build the image with `docker build . --tag fastingress:latest` and make sure you have a running Kubernetes cluster with `kind`. In the cluster, apply the `kubernetes/nginx-deployment.yml` file. Make sure the service is up. Then, make sure the `fastingress` image is loaded by `kind load docker-image fastingress:latest`. 

## Configuring the contorller
Apply the `kubernetes/ingress.yml` file in the cluster, and enable the proxy by `kubectl proxy`. Then check if the nginx service is reachable on `http://127.0.0.1:8001/api/v1/namespaces/default/services/nginx-service:80/proxy/`. 