#!/bin/bash

set -e
if [[ ! -f ./ferris-ci ]]; then
    build_ferris_ci="$(dirname -- "$0")/build_ferris_ci.sh"
    $build_ferris_ci
fi

build_dir="circleci-junit-fix-src"
VERSION="0.2.2"
git clone --depth 1 --single-branch --branch "v${VERSION}" https://github.com/conradludgate/circleci-junit-fix.git "${build_dir}"
cd "$build_dir"

CARGO_PROFILE_RELEASE_OPT_LEVEL="z" \
	ARGO_PROFILE_RELEASE_PANIC="abort" \
	CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1" \
	RUSTC_FLAGS="-C strip" \
	cargo build --release --target x86_64-unknown-linux-musl

mv target/x86_64-unknown-linux-musl/release/circleci-junit-fix ../circleci-junit-fix
cd ..
./ferris-ci upload -z tar-gz -o "circleci-junit-fix-${VERSION}.tar.gz" ./circleci-junit-fix
rm -rf $build_dir
rm circleci-junit-fix
