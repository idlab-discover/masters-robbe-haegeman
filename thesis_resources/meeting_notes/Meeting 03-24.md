# Notes - Meeting 24/03/25

## Preparation

### Current progress

- Update projects + dependencies
- Finish implementation of automatic watches
  - `get`, `list`, `delete` and `patch` methods
    - Update dynamic objects
  - Generalize the `setup_watches` from only secret to each kind specified
  - Rename secondary resource storage to `cache_secondary`
  - Swap to using the [`async_trait` crate](https://docs.rs/async-trait/latest/async_trait/)
    - Ultimately no reason not to
      - Async traits are a very hard topic[^1][^2] -> likely take a while to stabilize
      - Gives us [*"`dyn` compatibility"*](https://doc.rust-lang.org/reference/items/traits.html#r-items.traits.dyn-compatible.async-traits) (formerly known as "object safety")
    - If later no longer required, easy to remove
- Work on API extension server
  - [More in depth writings](../findings/api_extension_server.md)
  - Docker image (based on Debian-slim due to advice against Alpine)
  - HTTPS required
    - Use of self signed certificates
      - Hard to implement due to immature state `rcgen`
        - Tutorials outdated
        - Documentation not always clear
    - Less setup required than extra files or cert manager
    - Not production ready approach however -> write about how to do it properly
  - Logging through the `tracing` crate
  - Current state: correctly picked up by cluster
    - `kubectl get --raw /apis/poc.sec.res.kinds/v1/health` works
    - `/apis/poc.sec.res.kinds/v1/` also provided with proper (but empty) implementation
- Fix CI
  - Checkout files before path filter (otherwise only works on PRs[^3])
  - Update MSRV since we use `edition=2024`
  - Disable docs-rs due to workspace use
    - PoC works with workspaces and thus `-p lib` for example
    - kube-rs-building-cronjob doesn't
- Add planning to root README

[^1]: [Fasterthanlime: Catching up with async rust](https://fasterthanli.me/articles/catching-up-with-async-rust)
[^2]: [Baby Steps: Async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/)
[^3]: [Github dorny/paths-filter: Issue - Getting error "not a git repository (or any of the parent directories)"](https://github.com/dorny/paths-filter/issues/60#issuecomment-754725112)

### Subjects to discuss

- Feedback: *automatic watches*
  - Is the caching system we have now valuable?
    - Keeps secondary resources in status
      - Accessible from Kubernetes itself
      - Could be useful in the *multiple kinds* implementation
    - Alternative: keep map of primary resource to secondary resources in operator
      - Would introduce state
      - Doesn't work if multiple controllers need the info
    - Alternative: only keep identifiers (names or UIDs) in the status
      - Smaller memory footprint
      - Less data already available
  - Is a safeguarding system required?
    - When attempting to update the demo, my test of continuously updating the secret caused the operator to constantly reconcile due to its own changes
    - This means that using our library would require adding `requeue_after` actions
    - Do we have to attempt to account for this?
    - Looks out of scope and very janky
- Depth of Kubernetes topics in thesis writing
  - The API server stuff is very interesting and not that easy
  - It is however also not directly something I created
  - Previous advice was the level of my fellow student Computer Science Engineering
    - Most however, don't go further into Kubernetes than knowing its name
- Github Actions
  - Would advice against current approach
    - Reusable workflows + path filters only work for almost identical projects
    - Quickly falls apart when differences arise (see [Fix CI from "Current Progress"](#current-progress))
  - Will keep it for "history" sake
  - Would just copy for each individual project
    - (and maybe retire those which are no longer maintained in the WASM operator repo)

## Meeting

### Feedback: *automatic watches*

Cache is definitely ok, since it has a functional use.  
We're also talking about KBs in manifest data.  
Could be interesting to see impact however.

In case possible: create two implementations:

  1. Secondary resources in status
  2. Identifiers of those resources in status

Test its impact during the evaluation phase.  
Do not put too much work into this.

### Safeguarding system

Is not required, can be left out

### Depth of Kubernetes topics in thesis writing

Start from the knowledge of fellow students who have followed the `Cloud Storage and Computing` course.

### Github Actions

Good to talk about in the thesis writing.

Feedback often heard is to call a script ASAP with Github Actions.
Allows for better control than a YAML.
Those are often better suited for orchestration / interacting with PRs or issues.

### Next steps

- Finish API
- Separate reconcile expansion?
  - Advice against my original idea for this topic
  - Smaller scope than original
  - Requires further brainstorming
- Kubebuilder support
  - Look if also portable to Kubebuilder
    - Why / Why not?
  - Expand further on comparison `kube.rs` vs `Kubebuilder`

### Submission final info thesis

- **Title:** "Improving event-based Kubernetes controller APIs"
- **Language:** English (UK)
- **Submission period:** June
