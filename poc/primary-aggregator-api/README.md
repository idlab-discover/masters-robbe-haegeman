# Extension API server

Please read [extension API server](../../thesis_resources/findings/extension_api_server.md) for more info

## Instructions for testing

**Setup**

From the workspace root directory (so not within `primary-aggregator-api`)

```sh
docker build -f primary-aggregator-api/Dockerfile -t primary-aggregator-api .
minikube start
minikube image load primary-aggregator-api:latest
kubectl apply -f ./primary-aggregator-api/manifests/api_server.yaml
```

**Debugging**

```sh
kubectl describe apiservice v1.primary-all
kubectl get --raw /apis/primary-all/v1/health
kubectl get pods -l app=primary-aggregator-api
kubectl rollout restart deployment -n default primary-aggregator-api # Doesn't work since we have to set `imagePullPolicy: Never`
```
