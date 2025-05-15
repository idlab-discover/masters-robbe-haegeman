# Notes - Meeting 11/04/25

## Preparation

### Current progress

- Multiple kinds
  - Extension API server
    - Get primary resource route at: `/apis/primary-all/v1/{group}/{version}/{kind}/{namespace}/{name}`
      - Uses discovery API for fetching `APIResource` level of information
      - Uses discovery to get all possible `APIResource`s which are present
      - Gets the primary resource specified
      - Uses list + label filtering to get all secondary resources
      - Returns in JSON format: `{"prim_res": {...}, "sec_res": {...}}`
    - Error handling through anyhow
      - Automatic integration with Axum, allowing for use of `?` operator
        - Reports context to the user
        - Status code: 500 - Internal Server Error
          - Other status codes would get parsed and modified by Kubernetes in unclear ways
  - PoC integration
    - Create `get_primary` method
      - Uses API extension route
      - Works in the same way as *Kube.rs*'s `get`
      - Returns item of `Self` (i.e. a Rust type, e.g. your CustomResource), with the secondary resources stored in the status, inside a `Vec<DynamicObject>`
    - Create test for `get_primary()`
      - With tracing subscriber at `DEBUG` level for complete overview of actions
      - Ignored by default since uses k8scontext
      - Uses dummy DB custom resource
      - Creates DB, uses it as a primary, and reports the values
      - Currently mostly a smoke test, asserts are soon to follow
- Clean up of PoC
  - Restrict trait to only be applicable to `HasStatus` structs
    - Does not eliminate any useful resources, since `kube::API::Object` implements this
    - Required if we want to store the cache inside `etcd`
    - Allows for easier implementation
      - `initialize_status()` can now be provided by us
      - Make `cache_secondary(_mut)()` always return a value
      - Let user provide `cache_secondary(_mut)_status_dependent()` which returns an option based on whether the status is present
  - Shorten restrictions
- Transition to tracing everywhere in PoC
  - `tracing` is the de facto crate in the Tokio ecosystem for logging (they call it application-level tracing)
    - Allows you to write your own subscribers -> easy interaction with prometheus and such
  - Already used in the frameworks we use (e.g. *Axum*, *Kube.rs* (or at least the dependencies it uses))
  - Moving to it ourselves from logging allows us to avoid setting up the translations and use the more powerful framework
  - (was incredible helpful in the testing of `get_primary()` since it showed the requests being executed)

### Subjects to discuss

- Is current implementation fetching secondary resources proper?
  - i.e. currently, it looks for all kinds, then uses `list` request for each of them
  - Something like a register route could also be possible to avoid requesting to unused kinds?
- What are the current priorities
  - Writing
  - Extensions
    - Separate reconciliation
      - Would drop this due to concerns posed in previous meetings
    - Kubebuilder support
      - Keep it theoretical
  - Documentation of PoC
    - What each function does
  - Finishing of PoC
    - Feature flag to allow status to be stored in `etcd`
- Can I write about [service catalog](https://github.com/kubernetes-retired/service-catalog) (and its retirement)?
- Next meeting?

## Meeting

### Feedback test

> I was talking about my experience with `pytest` which would allow a cleaner interface for the tests through its fixture system

Keep the amount of boilerplate in each of the tests high:

- Normally, the reason to clean up code is to allow for easy modification, but tests are things that should not have to be modified
- Adding indirections can reduce the readability
  - When looking at OSS, tests can often be used to understand the interface
  - It is code that will always* work
  - -> Keep it simple

### Feedback code

The name `get_primary()` is not very clear; maybe something like `get_latest_with_secondaries()` is more representative.

### Extension API server

Good way of implementing it;
Register route wouldn't be the way to go, maybe the extension server can figure it out through the other requests made?
Wouldn't change the current method however;

This is being developed from the PoV of adding this to the Kubernetes API server.
The performance issues that could arise from the current implementation (due to being an extension server) wouldn't exist.

### Service catalog

Merlijn's PhD was about abstractions in Infrastructure as Code (IaC).
In traditional programming, these abstractions are things like functions and classes.

In Kubernetes, there is no concept of information hiding.
You always have the same interfaces for each tool.

This has the advantage that it allows tools to seamlessly integrate with one another.
For example, both tools created before and after ArgoCD, are supported by ArgoCD.

The disadvantage is that there is no built in information hiding.
Say we want a Ceph cluster running inside pods.
I create a CR and it creates a ton of secondary resources to create my requested cluster.
Attempting to keep overview of all of those resources, combined with the resources you created can be a pain.
Say you want to check the pods, well for something you deployed, you would have to check those pods or maybe a deployment, but for the Ceph cluster, you would need to CR.

The current implementation of *Fetch multiple kinds* has the building blocks to start work on information hiding.
This would allow a sysadmin to request a status of the highest level object that created the secondary resources.
That would solve one of the big issues in Kubernetes deployments: keeping a clear overview of everything that is running.

### Priorities

Agrees that it is a good time to start writing.
What they usually recommend is completing the written part for 80-90% before adding extras.

Implementation is currently already at a very good level, but there is still a lot of work when it comes to writing.
Try to bring it to a point where it would be finished if you spent another 2-3 days on it.

### Next meeting

Schedule it in Outlook so that everyone can be included, and set it for two weeks from now.
