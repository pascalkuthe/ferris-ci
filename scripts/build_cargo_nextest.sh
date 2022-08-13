#!/bin/bash

set -e

if [[ ! -f ./ferris-ci ]]; then
    build_ferris_ci="$(dirname -- "$0")/build_ferris_ci.sh"
    $build_ferris_ci
fi

build_dir="cargo_nextest_src"
devel="1"
VERSION="0.9.35"
if [[ $devel == "1" ]]; then
    BRANCH="main"
    URL=https://github.com/pascalkuthe/nextest.git
    VERSION="$VERSION-dev"
else 
    BRANCH="cargo-nextest-${VERSION}"
    URL=https://github.com/nextest-rs/nextest.git
fi

git clone --depth 1 --single-branch --branch "${BRANCH}" "${URL}" "${build_dir}"
cd "$build_dir"

CC_x86_64_unknown_linux_musl=clang${llvm_postfix:-} \
AR_x86_64_unknown_linux_musl=llvm-ar${llvm_postfix:-} \
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld" \
CARGO_PROFILE_RELEASE_OPT_LEVEL="z" \
CARGO_PROFILE_RELEASE_PANIC="abort" \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1" \
CARGO_PROFILE_RELEASE_LTO="true" \
RUSTC_FLAGS="-C strip" \
cargo build --release --target x86_64-unknown-linux-musl -p cargo-nextest

mv target/x86_64-unknown-linux-musl/release/cargo-nextest ../cargo-nextest
cd ..
rm -rf $build_dir
./ferris-ci upload -z tar-gz -o "cargo-nextest-${VERSION}.tar.gz" ./cargo-nextest
rm cargo-nextest
