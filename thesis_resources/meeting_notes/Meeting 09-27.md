# Notes - Meeting 27/09/24

## Preparation

### Current progress

- General research in the use of Kubernetes, with a focus on operators.

### Subjects to discuss

- Introduction to the project
  - What is the goal?
  - How to communicate?
  - What should be an initial focus?

## Meeting

### Objective

Extend Kubernetes API to support an **event-based API** in Golang. Focus on **custom resources** and build a **POC** to later expand into the control plane.

### Key Points

- **Operator with Custom Resources**: Needs to work in an **event-based** way.
- **POC by End of 1st Semester**: Should solve a part of the problem and lay the foundation for further expansion.
- **Research Reference**: Check Kevin’s thesis for insights.

### Questions to Explore

- **What do operators do when activated?**
- **Why aren’t existing event-based components enough?**: Identify resources/properties that change frequently.
- **Event Handling**: How do events work on **client and server** sides? What can be offloaded to clients (e.g., informers)?

### Approach

- **Experiment or Study**: Choose to experiment or read documentation first.
- **Focus on Current Version**: Limit research to **current Kubernetes version**.
- **GitHub**: Relevant code is on [GitHub](https://github.com/idlab-discover/wasm-operator).

### Timeline

#### 1st Semester

- **Meetings**: Every 2 weeks.
- **Deliverable**: Working **POC** solving part of the problem by semester's end.

#### 2nd Semester

- **Meetings**: Every 1-2 weeks.
- **Focus**: Final development and benchmarks.
- **Final Goal**: Working system based on POC, solving core issues.

>[!NOTE]
> Important for **burgies**—POC must work by the end of the first semester.

### Next Meeting

- **Date**: Monday, October 7th, 10 AM
- **Task**: Prepare a detailed work plan.
