# Notes - Meeting 11/04/25

## Preparation

### Current progress

### Subjects to discuss

- Operator frameworks?
- Footnote or cite
- Shorten the other sections?
- What to do with WASI transition?
- Should programming language, cargo workspace etc. be architecture or implementation?
- How do you start such sections?

## Meeting

### Shorten the other sections?

Size is not a problem (unless the dissertation ends up being 200 pages).
Focus on the other sections.
If you have time left in the end, you can come back to these.
There is no reason to shorten topics, unless you are under time pressure.

### Footnote or citation?

Everything in citation, since it is both more consistent and it stores the dates.
Even the thesis' repository can be a citation.

In short, there is no reason to use footnotes.

### What to do with the WASM transition topic?

Spend one paragraph on explaining WASM / WASI and then focus on the changes made.

### General feedback

Both writing and content are good.

Not feedback on this thesis, but in general good to watch out for the following:

- No passive form
- No "we" use
- Only present tense (/ no past)

The figures, even if not discussed, should be made bigger.

The use of citations is good (as in there are enough of them).

#### Introduction

Get straight to the point, starting the introduction with Edge / IoT.
"Kubernetes would be interesting for this movement, but has these problems."

The main theme: reduce the overhead of the control plane through WASM operator,
but the operators that we wanted to adopt were not event based enough.
That is why I'm building...

#### Table of contents

Important cutoff point: study (e.g. background, state of the art) vs. own contributions
Often times very black and white.

Start with all the background chapters, then chapters surrounding own contributions.
Currently, these are quite interwoven.

##### Investigation of Percona MongoDB operator

> **Question from me:** the Investigation of Percona MongoDB operator for example, contains a Periodic Reconciliation part,
> would this have to be restructured so that it can be moved to the background chapters?
> Wouldn't this make it very random to be included?
> The reader would not know why it is relevant at that point.

It is not a big deal if the background provided does not have a lot of context.
Most of the time, the background section is skimmed by the reader, referring back to it if required.
It is often not read completely.

Periodic reconciliation fits well within the section however, due to it being integrated and not just purely focussed on background.
It fits well in the context and is focussed on the task.
Thus, it should probably be kept there.
Just move the entire "Investigation" section to another chapter (after the background).

#### WASM operator

WASM operator can be kept in the background chapters.
Once you reach the improving section however, it should probably be moved to a separate chapter (within own work).

> **Question from me:** where should it be placed? It feels a bit out of place and is hard to integrate in the "story".

Within the research questions seems like a good option:

- We see many issues
  - Why is it not easy to use
    - -> Dev experience
  - Why is it not the most efficient
    - -> Event based reconciliation

This subdivision should be mentioned, probably best at the end of the introduction.
The entire dissertation is about improving WASM operator, which both topics do in other ways.
This makes everything fit together.

As to where it should be moved:
it does not fit into architecture, since nothing changes on that front.
That means it should be implementation.

If we notice when it is finished, that the section is misplaced, then we can always just move it.

### Architecture

- Split in subchapters for each improvement
- Be very explicit on **what** you want to achieve
  - e.g. The point of this change is to make it easy to...
- Give an overview of how the entire system operates (non-specific)
  - e.g. the SDK connects to the API extension server, with a drawing, making it very clear about what this is (control plane, sdk of operator...)

It should be possible for someone else to take the architecture and implement it in their own, other way.
There is no reason why it wouldn't be possible to implement the extension server + SDK in Go for example.
Since Kubernetes is the main topic, it is not an implementation detail.
Instead, the technical details are all connected to Kubernetes, which should be clear from the drawing.

The goal is to provide a big picture overview explaining

- **What** was built
- **Why** it was built
- The **impact** it has

Readers should be able to skip the implementation section if they do not want details

Provide an architecture drawing for each section as well.
This should give an overview of the architecture of the inner piece.

A total overview of API server vs SDK, how they are integrated...
Something similar to ![overview from presentation](/attachments/overview_project_architecture_presentation.png),
but more integrated into the Kubernetes architecture context.

#### How to start

"In order to solve the research questions, we propose the following architecture."
Once the entire chapter is written, it will become clear if that is sufficient as an introduction to the chapter.

It is a standard structure / chapter, so everybody knows what to expect.
More than a single sentence should thus not be required.

Best to refer back to the introduction to start the chapter.

### Implementation

Explore in more detail e.g. through the use of sequence diagrams.

The programming language of choice is an implementation detail.

Can use the same structure as the architecture (i.e. split on the different sub-projects).s

#### How to start

"We have implemented the architecture"
Start by saying it was done in Rust and go from there.

### AI use

Has to be mentioned in the appendix + reasoning.

### Captions

The use of captions can be improved.
Mention the core idea people should get out of the caption.

For example, for the KubeEdge figure:
> KubeEdge architecture shows clear separation between cloud and edge components ensuring that expensive,
> resource intensive control plane functionality can remain in the cloud

It is a similar case for when people read articles / blogposts:
most will read the titles, look at the figures and read the corresponding captions, while the text is sometimes glossed over.
Those are thus very important.

### Citation

Currently, a lot of citations only display the URL.
[Zotero](https://www.zotero.org/) is recommended.
It has a Firefox extension, allowing you to save them as citations directly from the website.

A source has to at least have the date.
When code is referenced, you should still mention the date, owner of the code...
Authors are not often added, but it is a good thing to do.

Include the DOI link for papers, since this is important for citations.

#### Chinese citation

Add it as a site, with both title and author in Chinese.
Then add a note with "Translated using Firefox translate" (as is already present)
