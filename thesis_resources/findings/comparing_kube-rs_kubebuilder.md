# Comparing Kube.rs and Kubebuilder

Below is a list of some of the differences (or annoyances) in developer experience between Kube.rs and Kubebuilder.
The goal of this list is not to create a very strict comparison, but to give my view as a developer.
The notes can thus have a subjective undertone.

- **Nullability Assumptions**: Go often assumes nullable values are non-null, which is not allowed in Rust.
- **Time Formats**: Handling time formats is frustrating in both languages.
- **Indexing**: Kube.rs lacks support for "indexing".
- **Logging Library**: The logging library used in the tutorial for Kubebuilder is better suited for operators than any I know or have found in Rust.
- **API Objects**: Kube.rs works with API objects, which is often significantly more verbose.
- **Patching**: Patching is more elegant in Go, where generating a diff is easier.
- **Error Handling**: Rust's error handling using `Result` types and the `?` operator is much more readable than Go's approach.
