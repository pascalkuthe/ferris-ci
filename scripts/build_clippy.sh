#!/bin/bash

set -e

if [[ ! -f ./ferris-ci ]]; then
    build_ferris_ci="$(dirname -- "$0")/build_ferris_ci.sh"
    $build_ferris_ci
fi

if ! [ -x "$(command -v docker)" ]; then
	if ! [ -x "$(command -v podman)" ]; then
		echo 'Error: podman or docker is required.' >&2
		exit 1
	fi
	echo "found podman..."
	docker="podman"
else
	docker="docker"
fi



build_dir="clippy_src"
VERSION="1.64.0"
BRANCH="rust-${VERSION}"
URL=https://github.com/rust-lang/rust-clippy.git

git clone --depth 1 --single-branch --branch "${BRANCH}" "${URL}" "${build_dir}"

$docker run -v "$(pwd):/io:Z" --entrypoint /bin/bash rust:slim -c "
cd /io/${build_dir} && \
CARGO_PROFILE_RELEASE_OPT_LEVEL=z \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
RUSTC_FLAGS=\"-C strip\" \
cargo build --release"

mv $build_dir/target/release/clippy-driver ./clippy-driver
mv $build_dir/target/release/cargo-clippy ./cargo-clippy
rm -rf $build_dir
./ferris-ci upload -z tar-gz -o "clippy-driver-${VERSION}.tar.gz" ./clippy-driver
./ferris-ci upload -z tar-gz -o "cargo-clippy-${VERSION}.tar.gz" ./cargo-clippy
rm cargo-clippy
rm clippy-driver
