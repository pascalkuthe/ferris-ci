#!/bin/bash

set -e
mkdir -p /opt/rh
cd /opt/rh
ferris-ci download --decompress rh-python38.tar.zst

yum -y update
yum -y install \
	libgfortran \
    atlas \
    libquadmath
