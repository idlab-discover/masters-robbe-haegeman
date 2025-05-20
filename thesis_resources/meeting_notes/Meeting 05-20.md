# Notes - Meeting 11/04/25

## Preparation

### Current progress

- Improved *Introduction*
  - Modified RQ 1
- Modified / Expanded *Architecture*
- Implemented feedback *Tom* completely
- Implemented feedback *Merlijn* mostly
- Finished
  - *Implementation*
  - *Sustainability Reflection*
  - *Conclusion*
  - *Evaluation*
  - *Abstract*
  - *Future works*
- Update GitHub (see [https://github.com/idlab-discover/masters-robbe-haegeman/pull/35](PR #35))
  - Better documented in PR
    - Rename subprojects
      - `poc-secondary-resource-management` -> poc
      - `lib` -> `kube-primary`
      - `demo` -> `demo-controller`
      - `extension api server` -> `primary-aggregator-api`
      - `PrimaryResource` -> `PrimaryResourceExt`
    - Improve version handling
    - Move projects for clarity
    - Add workaround for `rustls` related issue
    - Update dependencies
- Started work on *Extended Abstract*
  - Finished *Introduction*
  - Setup citations using `.bib` file

### Subjects to discuss

- Questions
  - Citations before or after '.'
  - Always use the `kube-primary` and `primary-aggregator-api` names?
  - Use of Title Case
    - (Was feedback from other student)
  - Sources do not contain accessed on
  - Is Gabe Newell quote fitting?
- Feedback
  - New chapters
    - Discussion sequence diagram
  - Implementation of previous feedback
    - Architecture diagram
    - Modified titles
- Extended Abstract
  - Is this a paper or a summary
    - i.e. for students or researchers
  - What should be included

## Meeting

### Questions

- Citations before or after '.'
  - Follow IEEE rules
- Always use the `kube-primary` and `primary-aggregator-api` names?
  - Yes, also in titles and figures
- Use of Title Case
  - Does not matter
  - Is IEEE, but feels outdated -> we do not follow IEEE to the letter
  - Maakt hem niets uit tbh
- Sources do not contain accessed on
  - The urldate field does not matter, it is date that matters
  - Make sure that allows to find the version you used
  - If you can't find the last modified, use year of access
- Is Gabe Newell quote fitting?
  - Yes

### General

Use of passive form not yet removed -> use AI to highlight all passive

### Architecture

- Diagram very good
- Caption too long
  - Max 3 lines
  - Pref. 2 lines
  - Less important to change than the benchmarks
- Modify figures to use new naming scheme
- Implementation feedback looks good
  - No further remarks

### Evaluation

#### Benchmarks

- Currently none, since did not seem relevant to student.
  - would be a big improvement if added

##### Proposed

- Difference in memory usage (highest priority)
  - Let scale with number of used resources
  - Both for the controller and extension server (with extension server getting priority)
- Difference in latency
- Difference in amount of API request (low priority)
  - Can be used if easier
  - Less important than memory usage and latency

##### Reason

- What is the current cost of features?
  - i.e. is it required to integrate this into `kube-apiserver`?
  - If the limitations of the current implementation are low, then there is no reason to.
  - Since it is not implemented in `kube-apiserver`, you have to make sure it would be valuable to do so
- Can be discussed further in a *Discussion* subsection.

##### Cluster to use

- CloudNativeLab
  - Should have clusters available
  - Use Slack chat if you do not have a project yet

#### Current writing

- Can be improved
  - e.g. "While implementation is not yet complete"
    - Does it meet my requirements?
    - You should also explicitly state which requirements it does not meet

### Future work

- Content seems interesting

### Implementation

#### Sequence diagram

- Transition period with AI
  - Packaging of information is less relevant
  - Focus more on location of information

### Extended abstract

#### Content

- First add everything you have done
- If space left over, add some literature study
- Do not forget to add the Architecture diagram

#### Target audience

- Other researchers
- Write as if you would publicize it
- Literature and technology study thus not required
