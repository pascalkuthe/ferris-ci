#!/bin/bash

set -e
mkdir -p /opt/rh
cd /opt/rh
ferris-ci download --decompress rh-python38.tar.zst

microdnf -y update
microdnf -y --nodocs install \
	libgfortran \
    atlas \
    libquadmath
