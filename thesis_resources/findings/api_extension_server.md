# API extension server

Required due to [not being available natively](./fetching_multiple_kinds.md)
Discussed in [Meeting 10/03/25](../meeting_notes/Meeting%2003-10.md)

## Sources

- <https://kubernetes.io/docs/tasks/extend-kubernetes/setup-extension-api-server/>
- <https://metalbear.co/blog/writing-a-kubernetes-operator/>
- <https://docs.rs/axum/latest/axum/>
  - Axum because already in Tokio ecosystem
  - And used in tutorial
- <https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/apiserver-aggregation/>
