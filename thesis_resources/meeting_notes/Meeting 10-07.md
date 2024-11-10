# Notes - Meeting 07/10/24

## Preparation
### Current progress
- Read through reading material provided
- Create overview of sources
- Read up on operators + informers

### Subjects to discuss
- What part of the Kubernetes do we want to make "event-based"?
    - Changes to the api already get communicated with watchers in a sort of event-based way?
        - Not the correct event-based way?
            - Not with Go primitives?
            - Sharing the call-info -> having to poll the SharedInformer
            - The event not being passed -> forced to consider the whole state of the instance
        - Problem with the indirection of manager of resource  -> API server -> operator
        - Every state change is communicated instead of the specific relevant ones? (e.g. an operator logging errors doesn't care about more replicas)
    - Biggest prevention idle operator is checking state


### Information dump
#### Operators
[Source: Kubernetes docs](https://kubernetes.io/docs/concepts/extend-kubernetes/operator/)

#### Controllers
[Source: Kubernetes docs](https://kubernetes.io/docs/concepts/architecture/controller/)

#### Watchers
[Source: Kubernetes docs](https://kubernetes.io/docs/reference/using-api/api-concepts/#efficient-detection-of-changes)

> When you send a **watch** request, the API server responds with a stream of changes. These changes itemize the outcome of operations (such as **create**, **delete**, and **update**) that occurred after the `resourceVersion` you specified as a parameter to the **watch** request. The overall **watch** mechanism allows a client to fetch the current state and then subscribe to subsequent changes, without missing any events.

[Source: Redhat blog](https://www.redhat.com/en/blog/kubernetes-operators-best-practices)

> Operators are implemented as a collection of controllers where each controller watches a specific resource type. When a relevant event occurs on a watched resource a **reconcile cycle** is started.
>
> During the reconcile cycle, the controller has the responsibility to check that current state matches the desired state described by the watched resource. Interestingly, by design, the event is not passed to the reconcile cycle, which is then forced to consider the whole state of the instance that was referenced by the event.
> ...
> Level-based triggering, while arguably less efficient because it forces to re-evaluate the entire state as opposed to just what changed, is considered more suitable in complex and unreliable environments where signals can be lost or retransmitted multiple times.

### Informers
[Source: Go Package docs - Informers package](https://pkg.go.dev/k8s.io/client-go/informers)
[Source: Go Package docs - Cache package](https://pkg.go.dev/k8s.io/client-go@v0.31.1/tools/cache#SharedInformer)

> A SharedInformer maintains a local cache --- exposed by GetStore(), by GetIndexer() in the case of an indexed informer, and possibly by machinery involved in creating and/or accessing the informer --- of the state of each relevant object. This cache is eventually consistent with the authoritative state. This means that, unless prevented by persistent communication problems, if ever a particular object ID X is authoritatively associated with a state S then for every SharedInformer I whose collection includes (X, S) eventually either (1) I's cache associates X with S or a later state of X, (2) I is stopped, or (3) the authoritative state service for X terminates. To be formally complete, we say that the absent state meets any restriction by label selector or field selector.


## Meeting
### Research Status:
- Research is progressing well.

### Next Steps:
- Kevin observed that the traces from certain operators show that they schedule themselves to wake up again periodically. This behavior needs to be investigated and addressed.

### Main Discussion Points:
1. **Operator Waking Behavior:**
   - Why do real operators wake up every 15 seconds?
   - What exactly are they checking during these intervals?
   - Why is the current Kubernetes API not capable of handling this in an event-based manner?

2. **Event-Based Waking:**
   - It should be possible for operators to function in an event-driven model, where they only wake up if a certain condition or state change occurs.
   - However, some things change constantly, and waking up for every little change might not be useful, which could explain why the operator wakes itself every 15 seconds.
   - **Ideal Solution:** The operator should only wake up if a specific requirement or condition is met, reducing unnecessary wake-ups.

3. **Investigating Kevin's Operator:**
   - Review the traces from Kevin’s operator to understand why it reschedules or wakes up, and what checks it performs during these wake-ups.
   - The goal is to determine whether these checks can be made redundant.

### Future Work:
- Investigate **where the operator’s logic** is currently placed. This will be a key aspect of understanding its scheduling behavior:
   - Is the logic integrated into the Kubernetes API?
   - Or is it controlled by a local agent, such as the runtime?

### Action Items for the Next Meeting:
- Have a **clear understanding** of why these operators wake up periodically and why the current event-based API is insufficient for this purpose.
- Dive into the **code** and conduct trace analysis to gather insights.
- Start by analyzing Kevin’s operator, which exhibits this behavior.
   - Once understood, it will be interesting to look at other operators that may display different behavior patterns.

### Objective:
- By the next meeting, provide a detailed explanation of why operators start up periodically and why they do not use an event-based system with the current API.
