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

# for now it is easier to strip down the readhat python packges rather than building it ourself
# potentially more saving could be achived by a manual build (with Os) in the future
$docker run -v "$(pwd):/io:Z" ubi7/ubi-minimal bash -c "/io/scripts/extract_python.sh"

./ferris-ci upload rh-python38.tar.zst
rm rh-python38.tar.zst
