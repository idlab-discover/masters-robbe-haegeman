# Master thesis project - Improving event-based Kubernetes controller APIs

<div align="center">
  <img alt="project mascot: Ferris the crab holding the Kubernetes logo with a WASM backpack" src="./attachments/mascot.png" width="500" />
</div>

## Overview

This repository contains resources for my master's thesis focused on optimizing Kubernetes operators for edge orchestration using WebAssembly.  
Building on the prototype of Ramlot T. and Van Landuyt K., this project aims to find alternative solutions to wake-up behavior in operators, causing inefficiencies within the WebAssembly-based environment.
The chosen subject was *"Edge Kubernetes with WebAssembly"*, but the project evolved (partially) away from there.

## Background

This is the third instalment in a series of master's theses, where Kubernetes is adapted for orchestration at the edge by making use of WebAssembly.  
It builds on the prior work of Ramlot T. ([Github repo](https://github.com/thesis-2022-wasm-operators/wasm_operator)), who created the prototype, and Van Landuyt K. ([Github repo](https://github.com/kvanla/wasm-operator)), who expanded the prototype with predictive capabilities.

During the development of Van Landuyt's solution, problematic behavior was highlighted in the used operators, where the operators would wake-up in a set interval (this through analysis of the [Percona MongoDB Operator](https://github.com/percona/percona-server-mongodb-operator)).
This is problematic for the WASM prototype, since unloading is used to minimize the operator footprint.
Minimizing the number of wake-ups (i.e. calling the reconciliation function), increases the efficiency of the solution.

### "Accepted" reasons to use scheduled reconciliation

From [Kubebuilder book: Why not use RequeueAfter X for all scenarios instead of watching resources?](https://book.kubebuilder.io/reference/watching-resources.html?highlight=period#why-not-use-requeueafter-x-for-all-scenarios-instead-of-watching-resources):
> While RequeueAfter is not the primary method for triggering reconciliations, there are specific cases where it is necessary, such as:
>
> - **Observing External Systems:** When working with external resources that do not generate events (e.g., external databases or third-party services), RequeueAfter allows the controller to periodically check the status of these resources.
> - **Time-Based Operations:** Some tasks, such as rotating secrets or renewing certificates, must happen at specific intervals. RequeueAfter ensures these operations are performed on schedule, even when no other changes occur.
> - **Handling Errors or Delays:** When managing resources that encounter errors or require time to self-heal, RequeueAfter ensures the controller waits for a specified duration before checking the resource’s status again, avoiding constant reconciliation attempts.

### Source problematic behavior

In [findings/investigation_reconcile_percona_mongodb](./thesis_resources/findings/investigation_reconcile_percona_mongodb.md), the entire research process can be found, but in the end two main possibilities were found:

1. In order to support **sidecar containers**
   - These often have the ability to modify the behavior of an application, while not directly modifying the Kubernetes resources, preventing the Reconcile function from being called
   - This can also be seen under the category of "Observing External Systems"
2. In order to manage the **secondary resources** created when a CR is initialized
   - Doing this properly would involve setting up the correct watches, referencing the CR object and making sure the event filtering (through the use of predicates) is up to snuff

### Possible solutions

1. Add **CLI command to Kubebuilder** (and or operator-sdk) which creates the base for resource watching
   - Both frameworks normally used for project initialization, not during the project
   - Can't be done at initialization since the secondary resources are often added organically during development
   - Would have to add new command / use of framework
2. Modify / expand **Kubernetes API + Kubebuilder**
   - Register watches to all secondary resources using a single request
   - Has to be flexible -> allow for filtering some resources + use of predicates for each
   - Could for example add its own (optional) Reconcile function (e.g. ReconcileSecondaries), which could have it's own logic
     - Some stuff in the normal Reconcile function, doesn't have to be executed for secondary resources
     - In case an event is missed, a scheduled reconcile of e.g. 30 min should be sufficient (or using the `SyncPeriod` (in the manager options), since that is its purpose)
   - Could also add the ability to fetch all objects immediately
     - Reconcile always has to fetch the objects -> many GET requests
     - Would go against general Kubernetes behavior, but would still fit in their model of eventual consistency
     - Would have to think about how these objects are then represented (for example, would be handy to have the resources grouped by type or maybe even be able to choose in what format), but this could come from the use of a PoC

## Project structure

### External resources

- [Overleaf](https://www.overleaf.com/read/hskzbnjtxqfc#332172): the current version of the master thesis corresponding to this project
- [Github](https://github.com/idlab-discover/wasm-operator): repository of the official WASM operator project

All information is included in this repository. Everything relevant to the prototype will later be upstreamed to the WASM operator project.

### Internal folder structure

```text
+-- 📂operators                                     # Kubernetes operators created during learning.
|   +-- 📂kube-rs-building-cronjob                  # Naive translation of Kubebuilder tutorial to Kube.rs.
|   +-- 📂kubebuilder-building-cronjob              # Kubebuilder tutorial: "Building cronjob".
+-- 📂poc                                           # Proof-of-concept: both library and testing grounds.
+-- 📂thesis_resources                              # Resources and documentation specific to the thesis project.
|   +-- 📂findings                                  # Detailed research findings and analyses from the project.
|   +-- 📂meeting_notes                             # Summaries from bi-weekly thesis meetings.
|   +-- 📂investigation_reconcile_percona_mongodb   # Resources used in investigation scheduled reconciliation MongoDB operator
|   +-- sources.md                                  # Table containing overview of most of the used sources.
```

> [!NOTE]
> Meeting notes will often contain duplicate information. It is mostly used for tracking purposes.  
> The discussed topics are filtered and written down more thoroughly in the other parts of the project.

### Planning

#### Semester 1

| Period | Tasks |
| ------ | ----- |
| 27/09 - 07/10 | <ul><li>Go through previous master theses</li><li>Research what operators do when activated</li><li>Investigate how events work on client and server side and why these aren't enough for the WASM-operator</li></ul> |
| 07/10 - 21/10 | <ul><li>Analyze Van Landuyt's K.'s operators for the wake-up behavior by investigating the code and traces</li></ul> |
| 21/10 - 04/11 | <ul><li>Analyze Van Landuyt's K.'s operators for the wake-up behavior by investigating the code and traces</li><li>Compare the operator(/controller) architecture from Kubebuilder with kube.rs</li></ul> |
| 04/11 - 18/11 | <ul><li>Compare the operator(/controller) architecture from Kubebuilder with kube.rs</li><li>Investigate how scheduled reconciliation is implemented and why</li></ul> |
| 18/11 - 02/12 | <ul><li>Run the WASM-prototype locally and investigate further</li><li>Find which parts of the Percona MongoDB operator enforce the use of scheduled reconciliation</li><li>Continue learning Kube.rs</li></ul> |
| 02/12 - 11/12 | <ul><li>Work toward prototype solution</li><li>Work on presentation</li><li>Create first draft of thesis</li></ul> |
| 11/12 - 19/12 | <ul><li>Implement automatic watches in PoC</li><li>Implement feedback presentation</li><li>Create first draft of thesis</li></ul> |

#### Semester 2

| Period | Tasks |
| ------ | ----- |
| 10/02 - 24/02 | <ul><li>Implement automatic watches in PoC</li></ul> |
| 24/02 - 10/03 | <ul><li>Implement automatic watches in PoC</li><li>Improve K8s API integration</li></ul> |
| 10/03 - 24/03 | <ul><li>Improve K8s API integration</li><li>Separate reconcile functions</li></ul> |
| 24/03 - 07/04 | <ul><li>Separate reconcile functions</li></ul> |
| 07/04 - 21/04 | <ul><li>Kubebuilder support</li></ul> |
| 21/04 - 05/05 | <ul><li>Kubebuilder support</li><li>Benchmarks + Evaluation</li></ul> |
| 05/05 - 19/05 | <ul><li>Benchmarks + Evaluation</li><li>Extended abstract</li></ul> |

## Copyright

This project is released under the Apache License Version 2.0.
