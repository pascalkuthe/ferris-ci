#!/bin/bash

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

function build_image(){
    $docker build --tag "${1}:latest" -f "containers/${1}" .
}

function build_squashed_image(){
    $docker build --tag "${1}:latest" --squash-all -f "containers/${1}" .
}

build_squashed_image ferris_ci_build_x86_64-unknown-linux-gnu
build_squashed_image ferris_ci_build_win_x86_64-pc-windows-msvc
build_image ferris_ci_clippy
build_image ferris_ci_dep
build_image ferris_ci_fmt
build_image ferris_ci_test_runner_x86_64-pc-windows-msvc
build_image ferris_ci_test_runner_x86_64-unknown-linux-gnu
build_image ferris_ci_vendor
