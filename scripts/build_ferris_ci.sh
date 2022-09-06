#!/bin/bash

export CC_x86_64_unknown_linux_musl=clang${llvm_postfix:-}
export AR_x86_64_unknown_linux_musl=llvm-ar${llvm_postfix:-}
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

cargo build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/ferris-ci ./
