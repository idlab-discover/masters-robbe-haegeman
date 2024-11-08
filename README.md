# Master thesis project - Edge Kubernetes with WebAssembly
## Overview
This repository contains resources for my master's thesis focused on optimizing Kubernetes operators for edge orchestration using WebAssembly.  
Building on the prototype of Ramlot T. and Van Landuyt K., this project aims to find alternative solutions to wake-up behavior in operators, causing inefficiencies within the WebAssembly-based environment.

## Background
This is the third instalment in a series of master's theses, where Kubernetes is adapted for orchestration at the edge by making use of WebAssembly.  
It builds on the prior work of Ramlot T. ([Github repo](https://github.com/thesis-2022-wasm-operators/wasm_operator)), who created the prototype, and Van Landuyt K. ([Github repo](https://github.com/kvanla/wasm-operator)), who expanded the prototype with predictive capabilities.

During the development of Van Landuyt's solution, problematic behaviour was highlighted in the used operators, where the operators would wake-up in a set interval (this through analysis of the [Percona MongoDB Operator](https://github.com/percona/percona-server-mongodb-operator)).
This is problematic for the WASM prototype, since unloading is used to minimize the operator footprint.
Minimizing the number of wake-ups (i.e. calling the reconcilation function), increases the efficiency of the solution.

## Hypothesis
### Source problematic behavior
The current hypothesis suggests this behavior is in order to optimize operators receiving a large amount of events.
In case an operator has to watch resources with a large amount of events, calling the reconcile function each time an event occurs would become very inefficient.
This is why operators contain predicates, these are used by Controllers to filter Events before they are provided to EventHandlers (thus calling the reconcile loop).
The assumption is that this filtering is insufficient, creating the need for scheduled reconcilation.

### Possible solutions
Two possible solutions to this issue are currently being investigated:
1. Modifying the Kubernetes API to expand the event-filtering capabilities of the watch endpoint, allowing for behavior such as "the current value of the custom resource is an outlier" to be implemented.
2. Expanding the parent operator, used in the WASM-operator framework, to watch the events for the child operators, filter the necessary events, and only forward the minimal subset.

Currently, most of the research has focussed on the first solution, in order to gain a wider understanding of the problem.

## Project structure
### External resources
- [Overleaf](https://www.overleaf.com/read/hskzbnjtxqfc#332172): the current version of the master thesis corresponding to this project
- [Github](https://github.com/idlab-discover/wasm-operator): repository of the official WASM operator project

All information is included in this repository. Everything relevant to the prototype will later be upstreamed to the WASM operator project.


### Internal folder structure
#TODO


### Planning
| Period | Tasks |
| ------ | ----- |
| 27/09 - 07/10 | <ul><li>Go through previous master theses</li><li>Research what operators do when activated</li><li>Investigate how events work on client and server side and why these aren't enough for the WASM-operator</li></ul> |
| 7/10 - 22/10 | <ul><li>Analyze Van Landuyt's K.'s operators for the wake-up behavior by investigating the code and traces</li></ul> |
| 22/10 - 4/11 | <ul><li>Analyze Van Landuyt's K.'s operators for the wake-up behavior by investigating the code and traces</li><li>Compare the operator(/controller) architecture from Kubebuilder with kube.rs</li></ul> |
| 4/11 - 18/11 | <ul><li>Compare the operator(/controller) architecture from Kubebuilder with kube.rs</li><li>Expand the tracing within the operators to identify call sites of scheduled reconcilation</li></ul> |
| 18/11 - 2/12<br>2/12 - 16/12| <ul><li>Investigate the WASM-prototype</li><li>Work toward prototype solution</li></ul> |

## Copyright
This project is released under the Apache License Version 2.0.
