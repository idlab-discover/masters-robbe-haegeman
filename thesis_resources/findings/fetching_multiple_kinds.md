# Notes - Fetching multiple kinds in one API request

For the context of Kubernetes at the Edge, operators being able to fetch multiple kinds in one API request is helpful.

- Could allow watch routes which give the updates to all watched resources instead only the primary
- Could decrease amount of fetches

This goes against the REST structure Kubernetes normally deploys however, since this is a request for multiple kinds.
As can be seen below, the current `kubectl` method for fetching multiple resource types is just iterating over all and sending individual requests.

The reason `kubectl` is investigated here, is since it is the only Kubernetes tool I could find which returns multi-kind results.

## Kubectl: Current methods of fetching multiple data types

[Source: The Truth about “kubectl get all”](https://feloy.medium.com/the-truth-about-kubectl-get-all-49cb533d8b8d)

`kubectl get all,api-extensions` - This will return an overview of many of the resources within a namespace.

```text
all:
  pods (v1)
  replicationcontrollers (v1)
  services (v1)
  daemonsets (apps/v1)
  deployments (apps/v1)
  replicasets (apps/v1)
  statefulsets (apps/v1)
  horizontalpodautoscalers (autoscaling/v1)
  cronjobs (batch/v1)
  jobs (batch/v1)

api-extensions:
  mutatingwebhookconfigurations (admissionregistration.k8s.io/v1)
  validatingwebhookconfigurations (admissionregistration.k8s.io/v1)
  customresourcedefinitions (apiextensions.k8s.io/v1)
  apiservices (apiregistration.k8s.io/v1)
```

In short, even those 2 categories don't provide all resources.

### Observing the API calls made by `kubectl`

Using `kubectl get all --v=8` allows us to increase the verbosity level and thus reveal the request body from the requests made to the api server.
This sadly shows that even for just the general overview of each of the resources, kubectl just executes multiple API requests.

### Workaround for all resource types

[Source](https://stackoverflow.com/questions/47691479/listing-all-resources-in-a-namespace)

```sh
kubectl api-resources --verbs=list --namespaced -o name | xargs -n 1 kubectl get --show-kind --ignore-not-found -n <namespace>
```

This takes the overview of all api-resources from `kubectl api-resources` and pipes those `xargs` which executes kubectl get for each of the kinds returned.
This is thus not what we want.
