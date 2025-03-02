# Notes - Investigation scheduled reconcilation in other operators from OperatorHub

## Grafana operator

[Source: Grafana docs](https://grafana.github.io/grafana-operator/docs/overview/)
> **ResyncPeriod**
>
> Grafana doesn’t have any webhooks or similar ways of giving information to the operator that a Grafana resource, like a dashboard, has changed. Due to this the Grafana operator has to constantly poll the Grafana API to test for changes in the dashboard.
>
> To control how often this polling should occur, you can set the spec.resyncPeriod field. This field tells the operator how often it should poll the Grafana instance for changes.
>
> So, if for example, a dashboard has changed, the operator comes in and overwrite those settings after 5m by default. If you never want the operator to poll for changes in the dashboards you need to set this value to 0m:

> [!NOTE]
> At first glance it seems that the operator does use the RequeueDelay value (set to 10s) by default in the Reconcile function for the GrafanaDashboardReconciler, but this is not actually the case, since it is the one time where they check for a success and exit the code within that scope  
> What however is cool to see is that they use the bool success method that is highlighted in the [PR on the Percona MongoDB operator](https://github.com/percona/percona-server-mongodb-operator/pull/880)

## Elasticsearch operator

[Elasticsearch-operator/controllers/logging/elasticsearch_controller.go: Reconcile period](https://github.com/openshift/elasticsearch-operator/blob/a5c132efd4e0ce83541d8c15ea4df23454c79e1f/controllers/logging/elasticsearch_controller.go#L32-L36)

Both the "main" operator and the Kibana operator have the scheduled behavior.

The Kibana operator included in the code base, seems to have the proper watches + predicates setup. ([source](https://github.com/openshift/elasticsearch-operator/blob/a5c132efd4e0ce83541d8c15ea4df23454c79e1f/controllers/logging/kibana_controller.go#L249-L321))  
We could however once again explain this if we make the assumption that you would want your dashboard to update periodically  
(in this case, it would be less necessary due to the watches, but could for example cover missed events or moving the timeline...)

Behavior was once removed from the Kibana operator: [Github PR: Bug 1842865: Fix Reconcile loop return values](https://github.com/openshift/elasticsearch-operator/pull/379/)
