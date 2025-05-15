# API extension server

Currently, this is part of the example operator by [Metalbear](https://metalbear.co/blog/writing-a-kubernetes-operator/).
This is however temporary and is used to create a correct setup within a cluster first.

Please read [API extension server](../../thesis_resources/findings/api_extension_server.md) for more info

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
