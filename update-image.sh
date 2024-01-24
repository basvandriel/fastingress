# updates the image in kind cluster

docker build . --tag fastingress:latest  

kind load docker-image fastingress:latest

kubectl delete pod fastingress
kubectl apply -f ./kubernetes/ingress.yml