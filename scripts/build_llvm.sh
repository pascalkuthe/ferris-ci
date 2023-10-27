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


llvm_version="16.0.6"
$docker build --tag package_builder:latest -f containers/package_builder .
$docker run -v "$(pwd):/io:Z" package_builder:latest build llvm $llvm_version -j 8 
./ferris-ci archive llvm $llvm_version --upload --full
