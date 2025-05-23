# Benchmark `get_latest_with_secondaries`

```sh
minikube delete
minikube start
minikube image load primary-aggregator-api:latest
kubectl apply -f ../primary-aggregator-api/manifests/api_server.yaml
cargo run --bin 
