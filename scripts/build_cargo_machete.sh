#!/bin/bash

set -e

if [[ ! -f ./ferris-ci ]]; then
    build_ferris_ci="$(dirname -- "$0")/build_ferris_ci.sh"
    $build_ferris_ci
fi

build_dir="cargo_machete_src"
VERSION="0.3.1"
git clone --depth 1 --single-branch --branch "v${VERSION}" https://github.com/bnjbvr/cargo-machete.git "${build_dir}"
cd "$build_dir"

CARGO_PROFILE_RELEASE_OPT_LEVEL="z" \
	ARGO_PROFILE_RELEASE_PANIC="abort" \
	CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1" \
	RUSTC_FLAGS="-C strip" \
	cargo build --release --target x86_64-unknown-linux-musl

mv target/x86_64-unknown-linux-musl/release/cargo-machete ../cargo-machete
cd ..
rm -rf $build_dir
./ferris-ci upload -z tar-gz -o "cargo-machete-${VERSION}.tar.gz" ./cargo-machete
rm cargo-machete
