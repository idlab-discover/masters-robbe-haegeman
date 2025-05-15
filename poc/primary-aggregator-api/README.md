# Extension API server

Please read [extension API server](../../thesis_resources/findings/extension_api_server.md) for more info

## Instructions for testing

**Setup**

```sh
docker build -t primary-aggregator-api .
minikube start
minikube image load primary-aggregator-api
kubectl apply -f ./manifests/api_server.yaml
```

**Debugging**

```sh
kubectl describe apiservice v1.poc.sec.res.kinds
kubectl get --raw /apis/poc.sec.res.kinds/v1/health
kubectl get pods -l app=primary-aggregator-api
kubectl rollout restart deployment -n default primary-aggregator-api # Doesn't work since we have to set `imagePullPolicy: Never`
```
