# Benchmark Setup Summary

## `gke_antiaffinity`

- **Environment**: GKE (Google Kubernetes Engine), deployed via Terraform
- **Node Placement**: Anti-affinity
  - `benchmark` and `primary-aggregator-api` are scheduled on separate nodes (verified using `kubectl get pods -o wide -n poc-testing`)
- **Deployment Mode**: In-cluster execution
- **Status**:
  - **RESOURCES**: ✅ 25/05/25 - 00:40
  - **KINDS**: ✅ 25/05/25 - 01:00

---

## `gke_affinity`

- **Environment**: GKE, deployed via Terraform
- **Node Placement**: Affinity:
  - `benchmark` and `primary-aggregator-api` are co-located on the same node (verified using `kubectl get pods -o wide -n poc-testing`)
- **Deployment Mode**: In-cluster execution
- **Status**:
  - **RESOURCES**: ✅ 25/05/25 - 01:20
  - **KINDS**: ✅ 25/05/25 - 01:40

---

## `minikube_internal`

- **Environment**: Minikube (single-node)
  - Default settings (CPUs=2, Memory=3800MB)
- **Node Placement**: Affinity settings have no effect due to single-node setup
- **Deployment Mode**: In-cluster execution
- **Component**: `primary-aggregator-api` running inside Minikube
- **Status**:
  - **RESOURCES**: ✅ 25/05/25 - 13:30
  - **KINDS**: ✅ 25/05/25 - 13:40

---

## `minikube_external`

- **Environment**: Minikube (single-node)
  - Default settings (CPUs=2, Memory=3800MB)
- **Node Placement**: Affinity settings have no effect due to single-node setup
- **Deployment Mode**: Out-of-cluster execution
  - benchmark runs from the host using `kubectl`
- **Component**: `primary-aggregator-api` running inside Minikube
- **Status**:
  - **RESOURCES**:  ✅ 25/05/25 - 14:15
  - **KINDS**: ⏳ 25/05/25 - 14:30
