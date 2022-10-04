#!/bin/bash

set -e

apt-get update
apt-get install -y --no-install-recommends \
		ca-certificates \
		gzip \
		openssh-client \
		tar  \
		curl \
		gcc \
		libc-dev
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
ln -s /bin/ld /bin/ld.lld

# install cargo/rustc/clippy
ferris-ci install-rust "${RUST_VERSION}"
rustup component add clippy

mkdir -p /opt/rh
cd /opt/rh
ferris-ci download --decompress rh-python38.tar.zst

rm /usr/bin/ferris-ci