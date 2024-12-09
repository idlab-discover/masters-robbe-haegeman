# Notes - Meeting 21/10/24

## Preparation
### Current progress
- Research tracing solutions for Kubernetes operators
- Go through Kubebuilder guide and build custom operator
  - Found a nice visualization of the general controller setup ([Kubebuilder: architecture concept diagram](./attachments/architecture_concept_diagram.svg))
- Setup the Percona PostgreSQL operator
- Setup the ArangoDB operator
  - Local instance
  - Prometheus logging

Expected the ArangoDB operator's prometheus logs to contain information on **idle time** or overview of **when reconciles occur**, but this was not the case.  
Will instead look at the Percona MongoDB operator (the one used in Van Landuyt's thesis).

### Subjects to discuss
- What type of tracing did Van Landuyt use?
  - He states tracing, but this can mean multiple things:
    - Attach a debugger to the operator and record the function calls (or even execution tracing/program trace recording)
    - Using a tracing framework such as Jaeger or OpenTelemetry
    - Adding extra function calls to the source code that output to stdout or files
- The interim report
  - Not sure what to do with the schedule
  - What is the focus of the report

## Meeting
### Discussion Kube.rs
Compare the Kubebuilder architecture of operators against the Kube.rs architecture.
- How similar are they?
- Will the visualization correspond with Kube.rs as well?

The runtime requires knowledge of Kube.rs of course.  
The choice of when to switch my focus from Golang to Rust is mine.

### Answer tracing question
Van Landuyt went with the third option.
The tracing was adding print instructions displaying the current time within the `reconile` function.

### Report
#### Schedule
The schedule doesn't have to be very concrete.
It would be a 2-weeks sprint system and would be very hard to create.
It can however be very useful, especially to get a more concrete idea about goals to set.
Is however not required, due to the current pace of work looking good.

#### Focus
Can be used as a recap of everything that has happened to that point.
It will only be evaluated by the counsellors and supervisors of the project.

### Project structure

Everything related to the project -> [Github: Masters Robbe Haegeman](https://github.com/idlab-discover/masters-robbe-haegeman)
Everything still relevant after my thesis -> [Github WASM-operator](https://github.com/idlab-discover/wasm-operator)
