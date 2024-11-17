# Percona MongoDB operator - Tracing
[Percona docs: architecture](https://docs.percona.com/percona-operator-for-mongodb/architecture.html)

- Linux (KDE Neon 24.04)
- Minikube (.deb) v1.34.0 (commit: 210b148df93a80eb872ecbeb7e35281b3c582c61)
- Kubectl (snap) (Client: v1.31.2, Kustomize: v5.4.2, Server: v1.31.0)
- Operator version: 1.17

## Install instructions - Minikube
[Percona docs: Install on Minikube](https://docs.percona.com/percona-operator-for-mongodb/minikube.html)

### Setup of manifests
```sh
# Minimal requirements for the operator are higher than the minikube defaults
minikube start --memory=5120 --cpus=4 --disk-size=30g
# Deploy the operator using the following command:
kubectl apply --server-side -f https://raw.githubusercontent.com/percona/percona-server-mongodb-operator/v1.17.0/deploy/bundle.yaml
# Deploy MongoDB cluster with:
kubectl apply -f https://raw.githubusercontent.com/percona/percona-server-mongodb-operator/v1.17.0/deploy/cr-minimal.yaml
```

>[!NOTE]
> A backup of the manifests used during testing can be found in [manifests/percona_mongodb](./manifests/percona_mongodb/)

> [!TIP]
`minikube dashboard` is quite helpful during the startup sequence, since this can take some time (including failures).
### Usage testing
```sh
# List the Secrets objects
kubectl get secrets
# View the Secret contents to retrieve the admin user credentials.
kubectl get secret minimal-cluster-o yaml
```

The MongoDB database admin user and password can then be found in base64 encoded format (like any other secret).

> [!NOTE]
> These can be decoded using `echo <VALUE> | base64 --decode`

```sh
# Run a container with a MongoDB client and connect its console output to your terminal.
kubectl run -i --rm --tty percona-client --image=percona/percona-server-mongodb:7.0.12-7 --restart=Never -- bash -il

# Now run `mongosh` tool inside the `percona-client` command shell using the admin user credentials you obtained from the Secret
mongosh "mongodb://databaseAdmin:databaseAdminPassword@minimal-cluster-mongos.<namespace name>.svc.cluster.local/admin?ssl=false"
```

## Adding modifications
### Location of the reconcilation function
This was not in the position where it resides by default in a Kubebuilder project.

TODO: Look if `operator-sdk` uses another default position or if this is just a Percona customization

It can be found in [pkg/controller/perconaservermongodb/psmdb_controller.go](https://github.com/percona/percona-server-mongodb-operator/blob/82a1d9717b2854b04dd7e0e0778caaf54de68f60/pkg/controller/perconaservermongodb/psmdb_controller.go#L234)

### Using a customized image for the operator
[Github: E2E tests](https://github.com/percona/percona-server-mongodb-operator/tree/main/e2e-tests)


The following instructions post the customized operator to Docker Hub. This is due to the build process assuming this. A public image is the easiest, since no management of images within minikube is required.

1. Download the operator source code: `git clone -b v1.17.0 git@github.com:percona/percona-server-mongodb-operator.git`
2. Set an environment variable to forward the make process to a different image: `export IMAGE=robhaege/percona_mongodb_custom:latest`
3. Run `make build` and `make manifests`  
   Note: `make help` is also very useful
4. Modify the generated `bundle.yaml`, `cw-bundle.yaml`, `cw-operator.yaml` and `operator.yaml` files by replacing the image with your provided IMAGE env var
5. Rerun the setup of manifests section above, but now using the local files:

```sh
# Minimal requirements for the operator are higher than the minikube defaults
minikube start --memory=5120 --cpus=4 --disk-size=30g
# Deploy the operator using the following command:
kubectl apply --server-side -f ./deploy/bundle.yaml
# Deploy MongoDB cluster with:
kubectl apply -f https://raw.githubusercontent.com/percona/percona-server-mongodb-operator/v1.17.0/deploy/cr-minimal.yaml
```
