# Ferris-CI

[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/47632)
[![CircleCI](https://dl.circleci.com/status-badge/img/gh/pascalkuthe/ferris-ci/tree/staging.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/pascalkuthe/ferris-ci/tree/staging)
[![CircleCI Orb Version](https://badges.circleci.com/orbs/pascalkuthe/ferris-ci.svg)](https://circleci.com/orbs/registry/orb/pascalkuthe/ferris-ci) 

A collection of tools useful for CI/CD for projects written in rust.

## Overview 

The project contains two components: a (static) binary and a set of docker images.

The binary includes the following functionality:

* fast (multipart) uploads to s3
* generating a hash for Cargo.lock that does not depend on workspace crates to allow dependency caching
* automatic release inspired by [release-please](https://github.com/googleapis/release-please) (WIP)
* downloading/installing various tools useful in CI (used internally by the docker images)

The docker images are based on [ubi7-minimal](https://catalog.redhat.com/software/containers/ubi7/ubi-minimal/) to allow a small footprint and maximum compatibility with enterprise deployments.
The gcc toolchain has been replaced with the (newest version of) clang/llvm toolchain as these are required by various tools.
Furthermore, the image contain, tar, gzip, ssh and git to allow compatibility with various CI systems.

Additionally, the images include the following tools:

* the rust toolchain including clippy and rustfmt
* [cargo-nextest](https://nexte.st/): a modern test runner
* [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov): coverage reporting
* [cargo-machete](https://github.com/bnjbvr/cargo-machete): find unused dependencies

Some additional more niche tools are present (see status section):

* developer libraries of llvm
* python 3.8 (including numpy): allow building/testing pyo3
* [circleci-junit-fix](https://github.com/conradludgate/circleci-junit-fix): compatibility with circleci juint format

The `ferris_ci_win` image contains all tools necessary to cross-compile rust programs to x86-64_windows_msvc.
A minimal wine version is also available to allow testing cross compiled builds.

A strong focus has been placed on retaining minimal images. All tools are heavily stripped to allow small image size.
The main docker image is roughly 400MiB compressed while the windows docker image is roughly 550MiB compressed.

For convenience a [circleci orb](https://circleci.com/orbs/registry/orb/pascalkuthe/ferris-ci) is available that uses ferris-ci to implement caching etc.

## Status

My long-term goal with this project is to make it a general purpose toolkit for rust CI.
However, right now the project primarily servers my needs and specifically the requirements for my ![OpenVAF](https://github.com/pascalkuthe/OpenVAF) compiler.
For example, the docker images include static libraries of LLVM, because these are required for building OpenVAF even tough most projects will probably not use them.

Therefore, this project is more of a template than a ready to use tool at the moment.
However, forking and adjusting the project and adjusting it to your needs should be straight forward

The ferris_ci_win image can not be published to a public docker repository because it contains the msvc buildtools which do not allow redistribution. 
I use a private [ghcr.io](ghcr.io) repository for my personal projects.
