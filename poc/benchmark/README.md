# Benchmark `get_latest_with_secondaries`

## Standard workflow

```sh
minikube delete
minikube start
minikube image load primary-aggregator-api:latest
kubectl apply -f ../primary-aggregator-api/manifests/api_server.yaml
cargo run -- -f ./results.jsonl -r 100 -k 1
```

## Benchmarking application

> Returned from `cargo run -- --help`

```text
Usage: benchmark [OPTIONS] --file-path <FILE_PATH> --resource-count <RESOURCE_COUNT> --kind-count <KIND_COUNT>

Options:
  -f, --file-path <FILE_PATH>            File to output the JSONL results to
  -r, --resource-count <RESOURCE_COUNT>  Number of resources to create
  -k, --kind-count <KIND_COUNT>          Number of distinct kinds to use (maximum of 1)
  -i, --iterations <ITERATIONS>          Number of iterations to test [default: 100]
  -c, --cleanup                          Remove the primary and its secondaries at the end
  -a, --append                           Append the results to the file instead of overwriting
  -n, --namespace <NAMESPACE>            Namespace to use [default: poc-testing]
  -h, --help                             Print help
```
