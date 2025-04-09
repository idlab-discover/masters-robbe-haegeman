minikube delete
# kubectl delete -f ./manifests/api_server.yaml
# minikube image rm api-extension-multi-kinds

minikube start
docker build -t api-extension-multi-kinds .
minikube image load api-extension-multi-kinds
kubectl apply -f ./manifests/api_server.yaml
kubectl apply -f ./manifests/test_resource.yaml
