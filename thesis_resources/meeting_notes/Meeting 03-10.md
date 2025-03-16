# Notes - Meeting 10/03/25

## Preparation

### Current progress

- Merge [`cargo component` PR](https://github.com/idlab-discover/wasm-operator/pull/7) in [WASM-operator](https://github.com/idlab-discover/wasm-operator)
- Merge [CI pipeline PR](https://github.com/idlab-discover/masters-robbe-haegeman/pull/2) in [Masters-Robbe-Haegeman](https://github.com/idlab-discover/masters-robbe-haegeman)
- Restructure PoC
- Improve / update docs
- Continue work on automatic watch methods
- Research fetching multiple kinds in one API call
  - [Findings file on the topic](../findings/fetching_multiple_kinds.md)

### Subjects to discuss

- How should I implement the other `..._secondary` calls?
  - `get` / `list`
    - Normally work through providing name, but this would defeat purpose of having _secondary call
    - Maybe add verification that label is added or parent has label?
    - Should they update the DynamicObject array or should this be shrunk?
      - DynamicObject are basically the entire Kubernetes object, but without the strict type of Rust
- Is `setup_watches` sufficient?
  - Currently, has to run at creation time of the controller, with the same predicates for each resource of a kind
  - Could look into changing this at runtime, but doesn't seem possible in `kube.rs` -> would require changes
    - Seems out of scope
- How would we implement fetching multiple kinds?
  - (bold is personal preference)
  - We can either request from `etcd` directly or go **through the API servers**
  - We can add this functionality through the WASM parent operator or an **API extension server**
  - Do these get transformed into Rust types or as DynamicObjects?
- How would we implement the separate reconcile functions?

## Meeting

### Is `setup_watches` sufficient?

Yes, this seems sufficient and would be out of scope to go further

### Implementation questions

There is no issue with leaving out features.
Some things that come up during development as possible angles, go too far.
They mention I seem to be able to identify these myself.

Stay focussed on what you want to accomplish.
Only implement it as far as required to get those results.
The goal is not to implement features for the sake of implementing them.

Look at the features you are proposing and attempt to identify which are most interesting.
Focus development on those. Set an order.
That way, you can avoid a situation where you weren't able to get to the interesting topics
due to the previous one being more difficult than expected.

#### Feedback on the separate reconcile functions

They believe it to be less interesting to pursue than *fetching multiple kinds*.
This is mostly due to bad experiences with a previous student project,
where unexpected overhead and miscommunication made the solution impractical.
This is due to the complexity in coordinating different operators.
It is often not worth it to break the ease of use of 1 operator which handles all coordination.

We can look at the current landscape for examples as well.
Edge computing often does not use microservices (or is at least not as keen on using them as cloud).
This is because microservices basically hide complexity, by adding complexity.
The end solution is never as efficient as a centralized one, due to the complexity and overhead in the solution.

#### Feedback on ideas surrounding implementation

- Through the **API servers**
- Through **API extension server**
- Attempt to use Rust types, but if out of scope, just go back to DynamicObjects

### How should I implement the other `..._secondary` calls?

There is no problem with providing those simple functions which just assert the label.
Extra code doesn't really matter
