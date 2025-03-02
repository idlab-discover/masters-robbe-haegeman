# Notes - Meeting 10/02/25

## Preparation

### Current progress

This section will talk about work done the final meeting from last year, till now.
This includes both the exam period (in which I was focussed on those) + vacation.
My vacation was spent enjoying free-time + learning more on Rust and Kuberentes, thus not many topics which have to be explained here.

- Planning
- Thesis
  - Introduction
  - Research Questions
- CI pipeline
  - based on [Jon Gjengset's Rust CI conf repo](https://github.com/jonhoo/rust-ci-conf)
    - dependabot
    - checks
    - scheduled
  - Currently supports all tests
  - Monorepo's weren't supported by the original configs and are hardly supported by Github Actions
    - They are a real pain
    - Goal is that changing which paths should be checked is easy -> use of reusable workflows
    - YAML hooks looked like a great solution, but these aren't well supported
  - Filtering has to be added so that not all checks run when only one of the monorepo projects is changed

### Subjects to discuss

- Feedback on the "Current Progress" topics
  - The RQ's
  - Further structure of the thesis
  - What is relevant to explain when it comes to Kubernetes and what not?
  - What to do with WASM operator

## Meeting

### Thesis

#### Feedback introduction

Not really discussed, mainly focussed on the topics below.

#### Research Questions

##### Feedback on the current RQ's

> Context:
>
> - **RQ.1:** How do the Rust and Go ecosystems around Kubernetes operator compare?
> - **RQ.2:** What are the accepted use-cases for scheduled reconciliation?
> - **RQ.3:** Why do operator developers often choose for scheduled reconciliation outside of those use-cases?
> - **RQ.4:** How can the developer experience be improved to avoid this workaround?
> - **RQ.5:** How can this solution be expanded to integrate further with the WASM prototype and decrease overhead?

- Research Question 2 and 3 are too similar
  - Swap their order
  - First WHY it is done
  - Hard to say if RQ.2 should be kept
    - Best practice doesn't matter if everyone does it wrong
    - It is however still important to mention it in text
  - If RQ.2 is "how does everyone do it", then RQ.3 is a better formulation
    - Remove the "outside of those use-cases?" however
- Merge RQ.2 and RQ.4
- Remove RQ.2 and RQ.5
  - RQ.2 for the reasons above
  - RQ.5 since it doesn't fit the current project anymore
- Rewrite RQ.4
  - Don't use "workaround", keep it more neutral
- Create a research question around the following:
  - WHY do developers use scheduled reconciliation
  - HOW do we make it so developers use it less

##### How to use the RQ's

They are often used as verticals within the thesis.
You refer back to them in each chapter.

Per RQ, you attempt to find what is already discussed in other research.
Create a chapter on the information you found.

> [!NOTE]
> This is part of the evaluation and is thus very important

#### Structure

- **Chapter: Technology Study**
  - How Kubernetes + reconciliation works
  - Start from the knowledge of my fellow students
  - Inform the reader, provide background

- **Chapter: Literature Study**
  - Sometimes included in the same chapter as the technology study
    - Likely the best approach here since everything is closely related
    - Often, when discussing Kubernetes, first a few technical aspects, then some paragraphs on literature
  - More focused on reviewing actual research
  - No issue if there are few papers included
  - Many developments in Kubernetes happen in the industry
  - Clearly differentiate:
    - Studies addressing research questions should be in a separate chapter
    - I specifically analyzed how operators function
  - For example, find papers on scheduled vs ...
  - What is Kubebuilder, and how does it fit in?
  - Research all different challenges and opportunities of Kubernetes at the edge
  - Use your introduction as inspiration
  - For example, the lack of isolation in WASM
  - Analysis of previous master's theses
  - WASI
  - Serverless control planes → others trying to make them event-driven
  - Analysis of how the control plane currently functions
  - What is reconciliation itself?
  - What can I find on the general internet versus in research papers?

- **Chapter: Comparison of Rust and Go**

- **Chapter: Reconciliation**
  - Standard cases in Kubebuilder
  - How Percona MongoDB operator has implemented it

- **Chapter: Architecture (Solutions)**
  - Deployment diagram
  - Design properties
  - Possibly multiple designs
  - More high-level than implementation
  - You have Kubebuilder, then a plugin within it, and the result is a file...
  - The major components, how they interact, and how they solve the problem

- **Chapter: Implementation**
  - Often includes sequence diagrams
  - Dives deeper into:
    - File format
    - Algorithms used
    - Where in the codebase integration happens
    - Which other libraries were used to solve the problem

- **Chapter: Evaluation**

#### Explaining Kubernetes properties

See also **Technology Study**.

If it is necessary for the reader to understand the topic, you can explain Kubernetes concepts.
The focus has to be on what YOU have created however, but showing Kubernetes concepts in diagrams in order to make it easier to understand is of course not a problem.

#### WASM operator

That can be mentioned in the **literature study/technology study**, describing the state of the project before the changes.

Then, in the following sections, describe how you modified it.
This will become clearer as the writing progresses.

Can be a **research question**:  
*"How can we make it easier for people to use the WASM-operator prototype?"*

However, this won't be a major focus in the research itself.  

- **Architecture** → Deployment pipeline  
- **Implementation** → How it was implemented  

Not something to start writing about first.
