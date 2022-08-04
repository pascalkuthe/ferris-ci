#! /bin/bash

set -e

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

if [[ ! -f ./ferris-ci ]]; then
    build_ferris_ci="$(dirname -- "$0")/build_ferris_ci.sh"
    $build_ferris_ci
fi

$docker build --tag package_builder:latest -f containers/package_builder .
$docker run -v "$(pwd):/io:Z" package_builder:latest build git 2.37.1 -j 8
./ferris-ci archive git 2.37.1 --upload
