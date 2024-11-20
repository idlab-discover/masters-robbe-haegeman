# Notes - Configuration schedule

## Controller-runtime
> This library is used by both Operator-sdk and Kubebuilder

### Instructing the manager to call the Reconcile function
[Controller-runtime docs: manager - Options](https://pkg.go.dev/sigs.k8s.io/controller-runtime@v0.12.3/pkg/manager#Options)

```go
	// SyncPeriod determines the minimum frequency at which watched resources are
	// reconciled. A lower period will correct entropy more quickly, but reduce
	// responsiveness to change if there are many watched resources. Change this
	// value only if you know what you are doing. Defaults to 10 hours if unset.
	// there will a 10 percent jitter between the SyncPeriod of all controllers
	// so that all controllers will not send list requests simultaneously.
	//
	// This applies to all controllers.
	//
	// A period sync happens for two reasons:
	// 1. To insure against a bug in the controller that causes an object to not
	// be requeued, when it otherwise should be requeued.
	// 2. To insure against an unknown bug in controller-runtime, or its dependencies,
	// that causes an object to not be requeued, when it otherwise should be
	// requeued, or to be removed from the queue, when it otherwise should not
	// be removed.
	//
	// If you want
	// 1. to insure against missed watch events, or
	// 2. to poll services that cannot be watched,
	// then we recommend that, instead of changing the default period, the
	// controller requeue, with a constant duration `t`, whenever the controller
	// is "done" with an object, and would otherwise not requeue it, i.e., we
	// recommend the `Reconcile` function return `reconcile.Result{RequeueAfter: t}`,
	// instead of `reconcile.Result{}`.
	SyncPeriod *time.Duration
```

As is visible from the explanation: it is not recommended to use this value in order to run your operator, only for safety.

### Reconcile again after X time
[Kubebuilder book: Reconcilation process](https://book.kubebuilder.io/getting-started.html?highlight=reconcile#reconciliation-process)


> Reconcile again after X time:
> ```go
> return ctrl.Result{RequeueAfter: nextRun.Sub(r.Now())}, nil
> ```

#### Why should you use it and why not?
[Kubebuilder book: Why not use RequeueAfter X for all scenarios instead of watching resources?](https://book.kubebuilder.io/reference/watching-resources.html?highlight=period#why-not-use-requeueafter-x-for-all-scenarios-instead-of-watching-resources)

> Why not use RequeueAfter X for all scenarios instead of watching resources?
>
> Kubernetes controllers are fundamentally event-driven. When creating a controller, the Reconciliation Loop is typically triggered by events such as create, update, or delete actions on resources. This event-driven approach is more efficient and responsive compared to constantly requeuing or polling resources using RequeueAfter. This ensures that the system only takes action when necessary, maintaining both performance and efficiency.
>
> In many cases, watching resources is the preferred approach for ensuring Kubernetes resources remain in the desired state. It is more efficient, responsive, and aligns with Kubernetes’ event-driven architecture. However, there are scenarios where RequeueAfter is appropriate and necessary, particularly for managing external systems that do not emit events or for handling resources that take time to converge, such as long-running processes. Relying solely on RequeueAfter for all scenarios can lead to unnecessary overhead and delayed reactions. Therefore, it is essential to prioritize event-driven reconciliation by configuring your controller to watch resources whenever possible, and reserving RequeueAfter for situations where periodic checks are required.


> **When RequeueAfter X is Useful**
>
> While RequeueAfter is not the primary method for triggering reconciliations, there are specific cases where it is necessary, such as:
>
> - Observing External Systems: When working with external resources that do not generate events (e.g., external databases or third-party services), RequeueAfter allows the controller to periodically check the status of these resources.
> - Time-Based Operations: Some tasks, such as rotating secrets or renewing certificates, must happen at specific intervals. RequeueAfter ensures these operations are performed on schedule, even when no other changes occur.
> - Handling Errors or Delays: When managing resources that encounter errors or require time to self-heal, RequeueAfter ensures the controller waits for a specified duration before checking the resource’s status again, avoiding constant reconciliation attempts.


## Kube.rs
[Github issue: set controller reconciliation period](https://github.com/kube-rs/kube/discussions/1371)
> It's up to you with the duration you pass to [`Action::requeue`](https://docs.rs/kube/latest/kube/runtime/controller/struct.Action.html#method.requeue)

[Kube.rs docs: Action: requeue](https://docs.rs/kube/latest/kube/runtime/controller/struct.Action.html#method.requeue)
> `pub fn requeue(duration: Duration) -> Action`
>
> Action to the reconciliation at this time even if no external watch triggers hit
>
> This is the best-practice action that ensures eventual consistency of your controller even in the case of missed changes (which can happen).
>
> Watch events are not normally missed, so running this once per hour (`Default`) as a fallback is reasonable.
