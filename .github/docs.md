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
- checks
  - semver - lint your crate API changes for semver violations (will be added if released to crates.io)

## Testing locally

These workflows were tested using [`nektos/act`](https://nektosact.com/)
This can be done as follows

```sh
act -l    # List the jobs
act       # Run all (warning a lot of output)
act -j <JOB> -b INPUT_DIRECTORY="<DIR>"    # Run a job from scheduled
act -j <JOB> --env INPUT_DIRECTORY="<DIR>" --env INPUT_MSRV="1.56.1"    # Run a job from check
```

TODO: check if it should always by a relative path
An example of `<DIR>` would be: "./operators/kube-rs-building-cronjob"

> [!NOTE]
> `-b` does the same as `--env` but doesn't allow multiple

Act can generate a lot of output when using it without filtering.
It will however produce the most realistic results.
We advice that in case you want to use it this way, that you pipe the output to a file and grep fro specific jobs like:

```sh
act | tee act.log
grep -F "[use-reusable-2/check/stable / fmt" act.log
```

### Current overview of succesful runs

This code is still in active development.
Below is an overview of the `act` output:

| Job                                                 | Success | Reason for failure                                          |
|-----------------------------------------------------|---------|-------------------------------------------------------------|
| use-reusable-{1,2}/rolling/ubuntu / nightly         |   ✅    |                                                             |
| use-reusable-{1,2}/rolling/ubuntu / beta / updated  |   ✅    |                                                             |
| use-reusable-{1,2}/check/stable / fmt               |   ✅    |                                                             |
| use-reusable-{1,2}/check/stable / clippy-1          |   ❌    | environment variable $REVIEWDOG_GITHUB_API_TOKEN is not set |
| use-reusable-{1,2}/check/beta / clippy-2            |   ❌    | environment variable $REVIEWDOG_GITHUB_API_TOKEN is not set |
| use-reusable-{1,2}/check/nightly / doc              |   ✅    |                                                             |
| use-reusable-{1,2}/check/ubuntu / stable / features |   ✅    |                                                             |
| use-reusable-{1,2}/check/ubuntu / 1.77.2            |   ✅    |                                                             |

> [!NOTE]
> The {1,2} value was added in post, since most jobs failed / succeeded in the same way in both dirs

In short the issues are:

- not setting the GITHUB_TOKEN
- not setting the directory for the external actions
