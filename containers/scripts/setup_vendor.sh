#!/bin/bash

set -e

apt-get update
apt-get install -y --no-install-recommends \
		ca-certificates \
		gzip \
		tar  \
		curl
rm -rf /var/lib/apt/lists/*


# Install minimal git version
VERSION=2.37.1
ferris-ci download "git-${VERSION}-x86_64-unknown-linux-gnu-OFF.tar.zst" --decompress
chmod +x /GIT/bin/*
mv /GIT/bin/* /usr/bin/
chmod +x /GIT/libexec/git-core/*
mkdir -p /usr/libexec/git-core/
mv /GIT/libexec/git-core/* /usr/libexec/git-core/
rm -rf /GIT

# Install cargo
VERSION="1.64.0"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/cargo-${VERSION}.tar.gz"
ferris-ci install-tool cargo $URL
