# Notes - Meeting 04/11/24

## Preparation

### Current progress

- Interim report created (took a lot longer than expected)
- Traces on the Percona MongoDB operator collected
  - Through same method as Van Landuyt
  - Seems insufficient to identify the problem, but does showcase it nicely
  - Information on how this was performed, can be found in the [reconcile testing operators](../reconcile_testing_operators/) folder
- Start experimenting with Kube.rs through the guides section
- Forgot to mention in the previous meetings that I'm also still reading Kubernetes in Action

### Subjects to discuss

#### Suggestions for next goals

- Expand the Github: Masters Robbe Haegeman with all information from my personal notes
- Expand the tracing on the Percona MongoDB operator
  - What calls the reconcile loop?
  - What causes the scheduled startup?
  - Can we customize this startup behavior? (Where do the 5s intervals come from?)
  - This should all happen in as general of a way as possible
    - In order to support other operators
- Run the WASM-operator locally
- Research architecture of Kube.rs operators
- Create POC

I would propose the expanded tracing + running the WASM-operator locally for the next sprint.

>[!NOTE]
> Github: Percona MongoDB operator: [bundle.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/bundle.yaml#L19731), [cw-bundle.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/cw-bundle.yaml#L19750), [cw-operator.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/cw-operator.yaml#L46) and [operator.yaml](https://github.com/percona/percona-server-mongodb-operator/blob/main/deploy/operator.yaml#L48) seem to reference a `RESYNC_PERIOD` variable set to 5s, which could be the one causing the behavior.
>
> - We have to test if this is the case.
> - We have to find what part of the code is responsible for setting this value.

## Meeting

### Remarks on the report

Would have been great to see the goals for the end of the semester outlined in the report.  
-> was not comfortable with this, due to not having a complete overview of how deep the scheduled wakeup is integrated

Was done during the meeting instead.

### Schedule

**Most important:** know where the problem resides -> traces have to be deep enough in order to explain why  
A first attempt to try and solve the problem can also be interesting (e.g. a hard coded solution)

Traces will make this more apparent.

### What operator should I try to solve the problem for?
>
> - **Percona operator**, since this is the one highlighted by Van Landuyt originally
> - **WASM-operator**, since it is the most relevant for the project
> - **Custom operator**, either in Go or Rust, since this allows me to create the minimal operator to produce the problem

**To which the answer was the following**:  
Try to get a good understanding of the problem in the next sprint, then you can use the remaining two sprints of the semester to attempt to solve it.  
This information can then be used to see which of the possible operators is the best choice.
Server-side could be difficult to prototype before the semester ends.
Client-side should be possible.

Do it in a way that is easy to do -> use that for a POC -> then reevaluate

### Sharing of information

- Adding the visualization from the Kubebuilder book, will not be problem (as long as sources are correctly referenced of course)
- It is my choice how far I want to go with sharing the information
  - The intuition of the counselor would be to put as much online as possible however

This will also be what I end up doing.
I want this repo to be a sort of "single point of truth" so that anyone wanting the full picture of project can just be referred to the repo.
This should also help avoid the problem the information being spread out.
