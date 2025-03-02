# Resources for investigation Percona MongoDB operator

These are the resources used in the investigation of the [MongoDB operator from Percona](https://github.com/percona/percona-server-mongodb-operator).

## Testing environment

The operators are currently investigated in the following environment:

- Linux (KDE Neon 24.04)
- Minikube (.deb) v1.34.0 (commit: 210b148df93a80eb872ecbeb7e35281b3c582c61x)
- Kubectl (snap) (Client: v1.31.1, Kustomize: v5.4.2, Server: v1.31.0)

While running:

```sh
minikube addons enable metrics-server
minikube start --memory=5120 --cpus=4 --disk-size=30g
minikube dashboard
```

> [!NOTE]
> The custom startup settings are due to the Percona operators surpassing the default limits
