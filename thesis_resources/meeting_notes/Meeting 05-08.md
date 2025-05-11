# Notes - Meeting 11/04/25

## Preparation

### Current progress

- Feedback previous meeting implemented
  - Reordering chapters
  - Expanding captions
  - ...
- Finish *"Chapter 6: Improving the WASM-operator developer experience"*
- Finish *"Chapter 4: Architecture"*

### Subjects to discuss

- Implementation feedback
- General feedback
- Prototype diagram
- How far should I go in the Implementation chapter?
  - Explain Rust topics?
- What I'll be doing next

## Meeting

### Notes on implementation previous feedback

#### Table of contents

- Looks good!

##### Title: Investigation Percona MongoDB operator (PSMDB)

- Change to *"Analysis ..."*  
  \+ make more descriptive: what are you researching in this chapter?

  **Proposal:** *"Analysing periodic reconciliation in production operator"*  
  (does not even have to mention the PSMDB operator, since this can be done in the text)
- Do not place the abbreviations in titles

#### Captions

- Looks good!

### 5: Investigation Percona MongoDB operator (PSMDB)

#### Introduction

- Watch out with using the passive form.  
  It is better to say: "the paper investigates"
- Connect the two paragraphs.
- In the introduction, specifically mention what you will be doing / researching.  
  *"This chapter investigates..."*  
  The point is not to mention what will be said, but to make clear WHY you are providing the information.  
  Why are you saying this and is that a problem that you will solve?
  *"In order to solve this problem, we have to investigate what to do."*

  Not every chapter has to start with an explanation on what will be discussed.

#### 5.1: Confirming reconciliation interval

- Too much passive form.  
  The reason this is frowned upon is because it hides who did what.  
  Only used when you specifically mention someone else did it.
- Go through the paper or let someone with a language background go through it in order to remove the use of passive language.
  Its use should almost never be required in a paper.
- *"For guidance on creating..."*, should probably be something that is put on Github, with it being preferably replaced with something on what you have used.
  *"This documentation was used in the creation of..."* or even just placing that on Github using a README or just within the code.

#### 5.2.1: Accepted use cases

- The use of "we" in *"We propose"* is correct there.
  As long as "we" is functional, it is perfectly valid to use it.
  The problem is with using it in *"we will do a*, *now we're going to do b"* contexts.
  You can often replace its use with *In this dissertation"* or something similar, but if that is not possible than just use "we"
- *"We will take an example"* is not correct and should probably be replaced with *"For example in..."*

#### 5.3: Reasoning behind the behavior

##### Introduction

- Also misses "the approach".
  Imagine you leave out the title, then you have no idea what the intro is discussing.
  Consider adding something similar to *"This section investigates the causes for the behavior using several different methods"*

##### 5.3.5: User reporting

- Rename to *"Previous attempts to fix"*, since it is less about people reporting the issue and more on how they attempted to solve it

##### 5.3.6: Conclusion

- Rename to *"Findings"*, since there is only one Conclusion in a masters dissertation and that is at the end of the document.

#### 5.4: Other operators

- Expand the title, since it is more about confirming the findings
- "The operators were selected" is passive

### 4: Architecture

- This is an example of a good intro, which explains what will be discussed
- The chapter itself is mostly structured around why, but it is better to structure around what

#### Architecture diagram

- Proposal structure:
  - Left side: patched version of Kube.rs with client
  - Connected to API extension server
  - With the extension server connected to API server
- The diagram would make it so the section would thus be repeats of what can be seen in the diagram
- Use the structure of *"we had this problem and we have solved it this way"*
  - This is e.g. missing from the API extension server chapter, since why do we use an API extension server?
- The chapters currently aren't very connected

### Implementation

#### Prototype sequence diagram

- Replace the use of textboxes with sentences to a few keywords
  - Allow you to make everything bigger
  - More readable in a small format
  - Can be further explained within the text

#### Level of detail

- Try to think about next year: what information do students need in order to understand the project.

#### Structure

- Use architecture diagram as a framework
- Should I give each part of the project a name and refer to them only by that name?
  - Yes, that is a very good way of doing so

### General

- Already happy about the writing.
  Feedback can be of a sufficient level, rather than just enumerating the basics.

### What I'll be doing next

#### Future work

- Specialization requirement
  - Other student working on using Prometheus to trigger reconciliation
  - Possibility for cross pollination?

#### Extended abstract

- Take your thesis and summarize each chapter in around three paragraphs
- That is why you should leave that as last
- Chances are quite high that this will be my first paper
