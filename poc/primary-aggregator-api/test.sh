minikube delete
# kubectl delete -f ./manifests/api_server.yaml
# minikube image rm primary-aggregator-api

minikube start
docker build -t primary-aggregator-api .
minikube image load primary-aggregator-api
kubectl apply -f ./manifests/api_server.yaml
kubectl apply -f ./manifests/test_resource.yaml
