# Notes - Configuration schedule

## Percona MongoDB operator
### Reference to the value in the manifests
Searching through the generated files resulted in the following:

- [bundle.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/bundle.yaml#L19731)
- [cw-bundle.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/cw-bundle.yaml#L19750)
- [cw-operator.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/cw-operator.yaml#L46)
- [operator.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/operator.yaml#L48)

These files contain a `RESYNC_PERIOD` env variable set to 5s, but it doesn't seem to be used (or it is at least not the value controlling the reconcile behavior).
TODO: look what sets the value and where it is used

### Finding the value in the code
[Code snippet](https://github.com/percona/percona-server-mongodb-operator/blob/82a1d9717b2854b04dd7e0e0778caaf54de68f60/pkg/controller/perconaservermongodb/psmdb_controller.go#L91-L105)

```go
return &ReconcilePerconaServerMongoDB{
  ...
  reconcileIn:            time.Second * 5,
  ...
}, nil
```


[psmdb_cntroller: Reconcile](https://github.com/percona/percona-server-mongodb-operator/blob/82a1d9717b2854b04dd7e0e0778caaf54de68f60/pkg/controller/perconaservermongodb/psmdb_controller.go#L237-L239)
```go
func (r *ReconcilePerconaServerMongoDB) Reconcile(ctx context.Context, request reconcile.Request) (reconcile.Result, error) {
	log := logf.FromContext(ctx)
	currentTime := time.Now()
	fmt.Println(currentTime)

	rr := reconcile.Result{
		RequeueAfter: r.reconcileIn,
	}
  ...
}
```

Changing the value confirms that this is what is controlling the scheduled behavior.
