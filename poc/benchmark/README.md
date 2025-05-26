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
  -f, --file-path <FILE_PATH>
          File to output the JSONL results to
  -r, --resource-count <RESOURCE_COUNT>
          Number of resources to create
  -k, --kind-count <KIND_COUNT>
          Number of distinct kinds to use (maximum of 5) Kinds used in order: [secret, pod, service, configmap, deployment]
  -i, --iterations <ITERATIONS>
          Number of iterations to test [default: 100]
      --keep-resources
          Do not remove the primary and its secondaries at the end
  -o, --overwrite
          Append the results to the file instead of overwriting
  -n, --namespace <NAMESPACE>
          Namespace to use [default: poc-testing]
  -d, --delay-after-creation <DELAY_AFTER_CREATION>
          Delay in seconds to wait after dummy resource creation [default: 0]
      --delay-after-request <DELAY_AFTER_REQUEST>
          Delay in seconds to wait after each request (so 2 times each iteration) [default: 0]
  -h, --help
          Print help
```

## Container

```sh
docker build -f benchmark/Dockerfile -t benchmark .
docker run -it -v "$PWD/results:/results" benchmark benchmark /results/result.json -r 0 -k 0
```

## GCloud setup

[Terraform Tutorial: Provision a GKE cluster (Google Cloud)](https://developer.hashicorp.com/terraform/tutorials/kubernetes/gke) was used for directions.

Kept as much default as possible
But move to single zone setup and increase Kubernetes version from 1.27 to 1.31

```sh
terraform init
# Warning: took around 12 minutes (40 when failing due to lack of available resources)
terraform apply
```

```sh
gcloud auth configure-docker

PROJECT_ID=$(gcloud config get-value project --quiet)

IMAGE_NAME_API=gcr.io/$PROJECT_ID/primary-aggregator-api:latest
docker tag primary-aggregator-api:latest $IMAGE_NAME_API
docker push $IMAGE_NAME_API

IMAGE_NAME_BENCH=gcr.io/$PROJECT_ID/benchmark:latest
docker tag benchmark:latest $IMAGE_NAME_BENCH
docker push $IMAGE_NAME_BENCH

gcloud components install gke-gcloud-auth-plugin
gcloud container clusters get-credentials $(terraform output -raw kubernetes_cluster_name) --zone $(terraform output -raw zone)
```

## Complete local testing workflow

```sh
minikube start
minikube image load primary-aggregator-api:latest
minikube image load benchmark:latest
```

## Execute benchmarks within cluster

Note that the image names will have to be updated when using the GCR.
The imagePullPolicy also has to be modified.

- `benchmark` -> `gcr.io/$PROJECT_ID/benchmark:latest`
- `primary-aggregator-api` -> `gcr.io/$PROJECT_ID/primary-aggregator-api:latest`

```sh
kubectl apply -f ./primary-aggregator-api/manifests/api_server.yaml
# For some reason, the job is able to fail if both are configured in the same manifest file :/
kubectl apply -f ./benchmark/manifests/setup.yaml
kubectl apply -f ./benchmark/manifests/job.yaml
kubectl cp poc-testing/<pod-name>:/output/results_resource_latency.jsonl ./benchmark/results_resource_latency.jsonl -c sidecar-keep-alive
kubectl cp poc-testing/<pod-name>:/output/results_kind_latency.jsonl ./benchmark/results_kind_latency.jsonl -c sidecar-keep-alive
```
