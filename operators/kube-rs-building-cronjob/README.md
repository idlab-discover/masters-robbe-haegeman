# Building Cronjob - Kube.rs
When attempting to learn Kube.rs, I found that the tutorials were often quite lacking when compared to Kubebuilder.
This is quite logical, since Kubebuilder is both more mature and more widely used.
It however also sparked the idea to follow a Kubebuilder tutorial ([Kubebuilder book: Building Cronjob](https://book.kubebuilder.io/cronjob-tutorial/cronjob-tutorial)) using Kube.rs.
This would allow me to compare some of the differences, while also improving my Kube.rs and Rust skills.

This was originally done in a private repo, but after it sparked some conversation, it seems more appropriate to make it public.
**The code is NOT meant to be perfect Rust code!**
Instead, the goal is to keep the structure as close to the Go code as possible.
This makes it easier to compare the two solutions and made it easier to create (since I'm still learning both Rust and Kube.rs).

In case someone is interested in making a better optimized version, don't refrain from creating one!

## Some sources used
https://github.com/kube-rs/controller-rs
https://github.com/kube-rs/kube/issues/479#issuecomment-895594074

## Running the operator
```sh
cargo run --bin crdgen
kubectl apply --server-side=true -f ./crd/cronjob.yaml # To avoid metadata.annotations size limit
kubectl apply -f ./samples/batch_v1_cronjob.yaml
RUST_LOG=info cargo run --bin controller
```
