# Notes - Meeting 24/02/24

## Preparation

### Current progress

- WASM operator isn't buildable
  - Upgraded to Rust 1.85 due to Rust edition 2024
  - Wasn't able due to `wasm32-wasi`
  - Deprecated since Rust 1.78, but dependency of WASM-operator
    - Because of stabelization of WASI 0.2
  - Moved to `cargo component`
  - Further notes available in ["transition to wasip"](../findings/transition_to_wasip.md)
- Progress on PoC
  - Automatic OwnerReference
    - Allows for use of the `owns` method on controller, which watches all resources of a given type which have a reference back
    - Tested through modifying owned secret value through `kubectl`
      - Causes infinite loop, since always modifies
      - Is a mistake in the code logic, not in the watch
      - Is however something that has to be kept in mind
  - Automatic watching
    - Currently through non-ideal solution
      - No API's are provided by Kube.rs to modify watches during runtime
      - Requires function call at controller creation time
      - Watches can be created at startup for all resources
- CI pipeline
  - Directory filtering has been setup for `checks`
    - Not for `scheduled` due to its goal of running periodically
    - Reduces readability and makes execution through `act` harder
  - Now ready for deployment
    - Not directly applicable to WASM operator, due to its relience on `cargo component`
      - Most projects will probably be able to compile to stable also
      - But testing the proper toolchain would be valuable

### Subjects to discuss

- Transition to WASIp
- Progress on PoC
- Current system CI

## Meeting

### CI pipeline

The only way to keep Github Actions clean and readable is trough the creation of a new tool.
The most important part is that they are safe + consistent when it comes to secrets.
It is often something you create once and never look at again.

### Transition to WASIp

The transition to the `wasi` create and cleaning up some of the original "hacks" which are now mature features are valid.
It is however (as I alluded to) out of scope for this project.

Keep track of these for in a "Future work" section, which should be included in the thesis.
