# Rust CI

## Introduction

This CI configuration is heavely based on [Jon Gjengset's Rust CI conf repo](https://github.com/jonhoo/rust-ci-conf),  
found through his ["Setting up CI and property testing for a Rust crate" video](https://youtu.be/xUH-4y92jPg?si=qcJJXctb9Evw8VP5).

Each of the current workflows was taken from this repository.
The main difference is the filtering on directories:
this repository contains several Rust projects and in order to save on compute time, we should limit execution to the right paths.

## Features

Enabled features:

- Dependabot: checks if dependencies can be updated in both Github-actions and cargo
- checks
  - fmt: checks that the code is formatted according to rustfmt
  - clippy: checks that the code does not contain any clippy warnings
  - doc: checks that the code can be documented without errors
  - hack: check combinations of feature flags
  - msrv: check that the msrv specified in the crate is correct
- scheduled: tests weekly if updating the dependencies of this crate to the latest available that satisfy the versions in Cargo.toml does not break this crate

This project will not enable all features present in the CI conf repo.  
This is due to its prototyping nature making it less important to use some of the features.

Removed features:

- Code coverage
- Safety checks
  - miri - detects undefined behavior and memory leaks
  - address sanitizer - detects memory errors
  - leak sanitizer - detects memory leaks
  - loom - Permutation testing for concurrent code [Crates.io: Loom](https://crates.io/crates/loom)
- Nostd
