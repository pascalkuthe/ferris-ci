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



build_dir="rustfmt_src"
VERSION="1.5.1"
BRANCH="v${VERSION}"
URL=https://github.com/rust-lang/rustfmt.git

# git clone --depth 1 --single-branch --branch "${BRANCH}" "${URL}" "${build_dir}"

# $docker run -v "$(pwd):/io:Z" --entrypoint /bin/bash rust:slim -c "
# cd /io/${build_dir} && \
# CARGO_PROFILE_RELEASE_OPT_LEVEL=z \
# CARGO_PROFILE_RELEASE_PANIC=abort \
# CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
# RUSTC_FLAGS=\"-C strip\" \
# cargo build --release --bin rustfmt --no-default-features"

# mv $build_dir/target/release/rustfmt ./rustfmt
rm -rf $build_dir
./ferris-ci upload -z tar-gz -o "rustfmt-${VERSION}.tar.gz" ./rustfmt
rm rustfmt
