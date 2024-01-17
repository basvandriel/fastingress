# Kubernetes API
## Accessing the API
Like stated on [the documentation](https://kubernetes.io/docs/tasks/administer-cluster/access-cluster-api/), use `kubectl proxy` to expose the cluster to your current machine. This will default to the `8001` port. Then the API is accesible via `http://localhost:8001`

## Accessing services
Say you found the service on e.g. IP `10.108.196.19`. This is an internal cluster IP, and can be accessed through that IP **only** from the cluster itself. When wanting to access that from outside the cluster, the Kubernetes API can be used. Use the proxy url and the name, port and namespace of the service to look that up like below.

```bash
http://localhost:9000/api/v1/namespaces/default/services/nginx-service:80/proxy/
``` 

In this case, it's using the `default` namespace and `nginx-service` which lives on port `80`.