# Notes - Investigation Architecture Operators

## Environment

- Linux (KDE Neon 24.04)
- Minikube (.deb) v1.34.0 (commit: 210b148df93a80eb872ecbeb7e35281b3c582c61x)
- Kubectl (snap) (Client: v1.31.1, Kustomize: v5.4.2, Server: v1.31.0)

## Investigated Operators

The operators below were taken from the thesis of Van Landuyt.

### Overview

> [!NOTE]
> Operator SDK uses Kubebuilder under the hood.  
> The info on architecture was found by either searching the docs or through the issues

- [Percona’s PostgreSQL operator](https://github.com/percona/percona-postgresql-operator)
    - Architecture: [Operator-sdk](https://github.com/operator-framework/operator-sdk)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Percona’s Mongodb operator](https://github.com/percona/percona-server-mongodb-operator)
    - Architecture: [Operator-sdk](https://github.com/operator-framework/operator-sdk)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Percona’s Xtradb-cluster operator](https://github.com/percona/percona-xtradb-cluster-operator)
    - Architecture: [Operator-sdk](https://github.com/operator-framework/operator-sdk)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [ArangoDB’s operators](https://github.com/arangodb/kube-arangodb)
    - Architecture: Custom
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Argo’s CD Operator](https://github.com/argoproj-labs/argocd-operator)
    - Architecture: [Operator-sdk](https://github.com/operator-framework/operator-sdk)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Elastic Cloud on Kubernetes operator](https://github.com/elastic/cloud-on-k8s)
    - Architecture: [Kubebuilder](https://github.com/kubernetes-sigs/kubebuilder)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Prometheus operator](https://github.com/prometheus-operator/prometheus-operator)
    - Architecture: Custom / Kubebuilder?
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
- [Bank-vaults’s operator](https://github.com/banzaicloud/bank-vaults)
    - -> was moved to [Bank-vault](https://github.com/bank-vaults/bank-vaults)
    - Architecture: [Kubebuilder](https://github.com/kubernetes-sigs/kubebuilder)
    - Framework: [Controller runtime](https://github.com/kubernetes-sigs/controller-runtime) + [client-go](https://github.com/kubernetes/client-go)
