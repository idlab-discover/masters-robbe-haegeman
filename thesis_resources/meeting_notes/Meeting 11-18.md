# Notes - Meeting 18/11/24

## Preparation
### Current progress
- Documentation published on Github, including:
  - findings
  - meeting notes
  - information on operators for testing
  - planning
  - sources
- Found code responsible for scheduled reconcilation in Percona MongoDB ([link](../findings/investigation_percona_mongodb_reconcile.md))
- Further research into how scheduled reconcilation is implemented and why you should or should not use it ([link](../findings/configuration_schedule.md))
- Further research into the architecture of Kube.rs operators and how they compare to the visualization from Kubebuilder
- (Further progress in following Kube.rs guides and reading the "Kubernetes in Action" book)


### Subjects to discuss
- I propose to look into using an [extension api server](https://kubernetes.io/docs/tasks/extend-kubernetes/setup-extension-api-server/)
  - Allows our solution to be independent of Kubernetes versions
  - Should we write it in:
    - Golang since it is the language of Kubernetes
    - Rust since it is the language of the prototype (and subject of [a Kube.rs recommended blog post](https://metalbear.co/blog/writing-a-kubernetes-operator/))
- Referring back to the [Meeting 11-04 notes](./Meeting%2011-04.md#what-operator-should-i-try-to-solve-the-problem-for)
  - Choose to create operator for PoC in Rust showcasing minimal code to exemplify the behavior
- Next on the agenda:
  - Running the WASM prototype locally (and adding documentation if required)
  - Continue learning Kube.rs (and during that learning look into the Kubebuilder visualization)
- When should I mail progress to every supervisor/counsellor?
- [Overleaf changed its policy](https://www.overleaf.com/blog/changes-to-project-sharing) causing only one Editor to be allowed at a time.
  - Does this mean that I only add Merlijn as a Editor?


## Meeting
### Feedback current progress + next steps
The current research was very focussed on how reconcilation happens (which is a good thing), but try to find out what specifically these operators due during such a scheduled reconcile.

- What happens during the reconcile that isn't event based (i.e. what isn't watch/event based)
- What stops working inside the operator if the schedule is removed
- Why do many operators showcase this behavior even though it is considered as something to avoid (according to the Kubebuilder book, see [findings: configuration schedule](../findings/configuration_schedule.md#why-should-you-use-it-and-why-not))

### Extension API server
Seems to agree that it is a good idea to look into if possible, choosing Rust or Golang is personal preference.

### PoC
Agrees that the minimal testing operator idea is good, but is important to still reference what real world behavior it models.
They also agreed that the use of comments within the operator can help with clarity.

### Next on the agenda
Seem to agree, but now analyzing the specific pain points causing the requirement for scheduled reconcilation is a priority.

### Mailing the progress
This is recommended by the counsellors in order to keep everyone involved in the project, up to date.
This should probably consist of the meeting notes (maybe summarized a bit more), planning and progress and should be communicated every one to two weeks.

### Overleaf
Overleaf moved to Merlijn's Overleaf. Do not have access to links yet however.

### Next meetings / presentation
2 meetings left in the semester before the presentation  
-> date dependent on Bruno's agenda  
-> definitely allowed to share presentation before the meeting for feedback
