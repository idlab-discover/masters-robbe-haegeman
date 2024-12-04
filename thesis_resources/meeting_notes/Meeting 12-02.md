# Notes - Meeting 01/12/24

## Preparation
### Current progress
- Follow the [Kubebuilder tutorial: Building Cronjob](https://book.kubebuilder.io/cronjob-tutorial/cronjob-tutorial) with kube.rs
  - Due to lack of good guides in the Kube.rs ecosystem
  - Took quite some time: tutorial (and a lot of Go operators) make assumptions on when a value isn't Null
    - Rust doesn't allow these assumptions -> have to be explicit
    - Translation between the different time formats can be annoying
  - Completely translated now, but not functional due to different behavior
- Run WASM-operator full_test/run_wasm.sh locally
  - Feedback
    - Bad documentation of files / run process:
      1. Installed a lot on my machine, but no communication on what  
         (having to run shell scripts that don't communicate at a glance what is happening, is a bad dev exp)
      2. Required dependencies, but had no documentation beforehand
      3. No mention of recursive git clone being required (knew this from previous projects I worked on, but for example was never part of a course)
      4. Didn't know what the end result would be or why many of the parts are required (e.g. what is the flask server doing)
      - All small issues, but make it a lot less inviting to use
    - Lots of linting errors -> `clippy` can fix most of those
    - Unclear where to go next
      - Other run script is '(old)'
    - Often good code documentation
- Upgrade / update [mongodbSpammer](https://github.com/idlab-discover/wasm-operator/tree/main/controllers/mongodbSpammer) in order to verify proper behavior of the Percona operator
- Further research in the need of scheduled reconcilation for the Percona MongoDB operator
  - See [the findings file on the topic](../findings/investigation_percona_mongodb_reconcile.md)


### Subjects to discuss
- Investigation of the scheduled reconcilation
  - End conclusion: the operator currently doesn't watch all resources controlled by the CR
    - It only manages the primary resources, not the secondary resources
    - Multiple PR's created to fix the issue [PR: 880](https://github.com/percona/percona-server-mongodb-operator/pull/880), [PR: 1068](https://github.com/percona/percona-server-mongodb-operator/pull/1068)
    - Other operators also fit in this view (see [investigation other operators](../findings/investigation_other_operators.md))
  - Also makes supporting sidecar containers easier
  - See further discussion in [the findings file on the topic](../findings/investigation_percona_mongodb_reconcile.md)
- Creating a PoC
  - How should the operator be created
    - Modify existing operator in the WASM-operator group
    - Create 'normal' Rust operator
  - What counts as a minimal reproduction?
- When is deadline for the first draft of the thesis?
  - Currently all my write-up is in the repo


## Meeting
### Building Cronjob using Kube.rs [(with Kubebuilder tutorial)](https://book.kubebuilder.io/cronjob-tutorial/cronjob-tutorial)
Is definitely an interesting part of the process.
Differences between creating an operator with Kube.rs vs. Kubebuilder (that go further than just syntactic) should be documented and could be incorporated with the thesis

-> Definitely interesting to continue the operator and document the differences

### Improving the dev experience of the WASM operator prototype
#### Installation process
You can be very opinionated with the execution of the operator.  
In case something can be improved -> create a PR
(on that note, you can definitely add the `percona_mongodb_event_creator` to the Github)

**IF** I want to improve the shell script, then...  
[ShellCheck](https://www.shellcheck.net/) is definitely recommended -> great feedback on keeping code clean and a lot of warnings

**BUT** removing the need for a shell script through a thorough README, would be a better solution
  - Content
    - Describing what to install
    - How to setup the parent operator
    - A per operator setup process for example:
      - through a cargo command in the operator directory
      - through a README for each operator
  - Why?
    - Shellscript makes it easy to run benchmarks, but is hard to explain the steps
    - README allows devs to know what they are installing and executing  
      -> Makes it easier to expand on the solution  
      -> Removes barrier from entry (what will it install and how can I remove it?)


#### Linting errors
Definitely something that should be fixed, since it is low effort.  
Create a separate commit for this however, so the history stays parsable.  
Document the dev environment used (in this case, that would be using `clippy` on save)

#### Improving the documentation
Making the application easier to use / setup is definitely important in the project
Could be part of the reason that there is a lot of interest, but not a lot of people using it / further developing the prototype.


### Investigation of the scheduled reconcilation
Secondary resources seems to be a proper conclusion.
The sidecar containers are also interesting, but not the main focus here.

Possibilities to solve the problem:
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


### Creating a PoC
PoC is very dependent on what is possible in the limited timeframe,
because of that an operator outside the WASM environment is definitely a good option.
This frees up time, that can be used to further develop the solution

Both a minimal Rust and minimal Go operator are valid prototypes  
-> chances high that the Go operator support will be finished sooner than expected  
-> can work with Jonathan

A solution using a separate file (thus client-side) is a good option.
This would allow for easy development of an interface and could be used to easily display the advantages the solution would have.


### First draft of thesis
Definitely recommended to create a first version.
Allows the counselors to review the writing, and support more in case that is required.

Not enforced, but having a general idea written down on the structure of the paper (for example a table of contents) and an initial introduction would be a very good result.

### Cooperation with the other students
A group was created to help eachother with the use of the WASM prototype.
I plan on trying to create the documentation in order to help the others in using the framework, but most of these things can be done cooperatively.
