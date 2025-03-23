# API extension server

Required due to [not being available natively](./fetching_multiple_kinds.md)
Discussed in [Meeting 10/03/25](../meeting_notes/Meeting%2003-10.md)

## Sources

- <https://kubernetes.io/docs/tasks/extend-kubernetes/setup-extension-api-server/>
- [Metalbear blog: Writing a Kubernetes operator](https://metalbear.co/blog/writing-a-kubernetes-operator/)
  - Company behind mirrord ([notable adopter of Kube.rs](https://kube.rs/adopters/#open-source))
  - Described as *"Operator with extension api-server"* by [Kube.rs - Guides](https://kube.rs/guides/)
  - Does it with some shortcuts
    - Does not setup the aggregation layer
    - Does not use proper TLS
    - Does not setup all verbs
    - Does not setup OpenAPI v2 or v3 (via /openapi/v2 or /openapi/v3), which Kubernetes looks up for each new APIService
- <https://docs.rs/axum/latest/axum/>
  - Axum because already in Tokio ecosystem
  - And used in tutorial
- <https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/>
- <https://kind.sigs.k8s.io/docs/user/configuration/>
- <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/control-plane-flags/>
