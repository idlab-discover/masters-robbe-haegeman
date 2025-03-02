# Transitioning to WASIp1 or WASIp2

The WASM-operator project is not compatible with the latest Rust toolchain versions.
This is due to the use of the now deprecated [`cargo wasi`](https://github.com/bytecodealliance/cargo-wasi) which uses the `wasm32-wasi` target under the hood.
The repo has been archived with the note: *"cargo wasi is deprecated, use cargo component instead"*.
The `wasm32-wasi` target has been removed since Rust 1.84 and was already deprecated in 1.78 ([source](https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html)).

Instead, new targets, namely `wasm32-wasip1` and `wasm32-wasip2`, were introduced.
These correspond to both the older v0.1 spec and the v0.2 spec of WASI.
The p2 spec was initially introduced as a "tier 3" target, but has since been promoted to a "tier 2" ([source](https://blog.rust-lang.org/2024/11/26/wasip2-tier-2.html)).

## Current solution

As indicated on the `cargo wasi` repo, using [`cargo component`](https://github.com/bytecodealliance/cargo-component) is the easiest solution.

`cargo component` is a cargo subcommand for creating WebAssembly components using Rust as the component's implementation language.
It is however not without its faults:

  - The install is quite a bit larger than `cargo wasi`
  - It is considered experimental and is *not* currently stable in terms of the code it supports building.
  - It doesn't use the `wasm32-wasip2` directly, instead using p1 and then adapting them through a built-in WASI adapter snapshotted out of the Wasmtime repository.  
    See [WASI support section](https://github.com/bytecodealliance/cargo-component/tree/e0e34c74091bf0f9726db36c03c5174373966e92?tab=readme-ov-file#wasi-support).  
    They plan to use the `wasm32-wasip2` target directly in the future however.

The `wasm32-wasip2` is important and already commonly used.
The [examples from Wasmtime](https://docs.wasmtime.dev/examples-rust-wasi.html) also recommend them for example.

## Alternative solution

In my opinion, the ideal solution would be to eliminate the use of third party tools (even though these are of course created by the ByteCodeAlliance themselves) and instead make use of "standard" cargo with the mentioned targets.
This is also the alternative approach `cargo component` talks about in its ["Relationship with wasm32-wasip2" section](https://github.com/bytecodealliance/cargo-component/tree/e0e34c74091bf0f9726db36c03c5174373966e92?tab=readme-ov-file#relationship-with-wasm32-wasip2).
It mentions the following:

> So for now, if you only need WASI interfaces, then the wasm32-wasip2 target and the wasi crate should work.  
> If you have non-WASI WIT interfaces, whether third-party WIT interfaces or your own custom WIT interfaces, then use cargo component.

Attempting to build to project using `cargo --target wasm32-wasip2` didn't work however.
It resulted in the following error:

```text
error: linking with `wasm-component-ld` failed: exit status: 1
  |
  = note: LC_ALL="C" PATH="...
  = note: some arguments are omitted. use `--verbose` to show all linker arguments
  = note: error: failed to encode component
          
          Caused by:
              0: failed to decode world from module
              1: module was not valid
              2: failed to resolve import `http-proxy-abi::request`
              3: module requires an import interface named `http-proxy-abi`
          

warning: `simple-pod-example` (bin "simple-pod-example") generated 1 warning
error: could not compile `simple-pod-example` (bin "simple-pod-example") due to 1 previous error; 1 warning emitted
```

This is due to how `cargo` isn't able to solve the linking to the `http-proxy-abi` with the current implementation:

  - [pkg/controller/src/abi/mod.rs - Func wraps](https://github.com/idlab-discover/wasm-operator/blob/410d4369d83a257e93ae7ebef2aac780c6826d21/pkg/controller/src/abi/mod.rs#L13-L14)
  - [pkg/kube-runtime-abi/src/requuestor.rs - wasm_import_module](https://github.com/idlab-discover/wasm-operator/blob/410d4369d83a257e93ae7ebef2aac780c6826d21/pkg/kube-runtime-abi/src/requestor.rs#L6-L9)

P1 also didn't compile, stating less clear errors.
Due to it now being the "legacy" option, it wasn't investigated as much.

These are all probably issues that can be solved by using [the `wasi` crate](https://docs.rs/wasi/latest/wasi/) as mentioned in the `cargo component`docs.
This however requires quite some rewriting, is probably not the only WASI interface related issue, and the `wasi` crate isn't documented all that well, probably due to just being the output of [`wit bindgen`](https://github.com/bytecodealliance/wit-bindgen/).
Since it already worked using `cargo component`, the call was made that the change would be out of scope for my thesis.
