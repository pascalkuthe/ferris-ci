#!/bin/bash
set -eux

# Install Basic packes
microdnf -y update
microdnf -y --nodocs install \
	ca-certificates \
	glibc-devel \
	gcc \
	openssh-server \
	openssh-clients \
	tar
# remove caches
microdnf clean all
rm -rf /var/cache/microdnf
rm -rf /var/cache/yum
rm -rf /var/cache/dnf
rpm -ev lua rpm rpm-libs libdnf microdnf libsolv
# we just need the crt files from gcc so we can remove the (large) gcc/ld binaries
rm /usr/bin/cc
rm /usr/bin/cpp
rm /usr/bin/gcc
rm /usr/bin/c89
rm /usr/bin/c99
rm /usr/bin/gcc*
rm /usr/bin/ld*
rm -rf /usr/libexec
rm -rf /usr/share/doc
rm -rf /usr/share/man
rm -rf /usr/share/man

rm /bin/awk
rm /bin/sed
rm /bin/gcov*
rm /bin/*gcc
rm /bin/ar
# rm /bin/rpm*
# rm -rf /usr/lib64/libdnf*
# rm -rf /usr/lib64/librpm*



# install llvm and use clang instead of gcc
VERSION="14.0.6"
ferris-ci download llvm-${VERSION}-x86_64-unknown-linux-gnu-OFF.tar.zst --decompress
chmod +x /LLVM/bin/*
mv /LLVM/bin/* /usr/bin/
mv /LLVM/lib64/clang /usr/lib64/clang
ln -s /LLVM/lib64/clang /usr/lib/clang # fix broken build...
ln -s /usr/bin/clang /usr/bin/cc
ln -s /usr/bin/clang++ /usr/bin/cpp
ln -s /usr/bin/ld.lld /usr/bin/ld
ln -s /usr/bin/llvm-ar /usr/bin/ar


if [[ -n "${CROSS_TARGET:-}" ]]; then
	rm -rf /LLVM
	ferris-ci download "llvm-${VERSION}-${CROSS_TARGET}-OFF.tar.zst" --decompress
    mv /LLVM/bin/llvm-config* /usr/bin/
fi
mv /LLVM/lib64/* /usr/lib64/
mv /LLVM/include/* /usr/include/
rm -rf /LLVM

# Install minimal git version
VERSION=2.37.1
ferris-ci download "git-${VERSION}-x86_64-unknown-linux-gnu-OFF.tar.zst" --decompress
chmod +x /GIT/bin/*
mv /GIT/bin/* /usr/bin/
chmod +x /GIT/libexec/git-core/*
mkdir -p /usr/libexec/git-core/
mv /GIT/libexec/git-core/* /usr/libexec/git-core/
rm -rf /GIT


# Install Rust
ferris-ci install-rust $RUST_VERSION


# Install sccache
VERSION="0.3.0"
URL="https://github.com/mozilla/sccache/releases/download/v${VERSION}/sccache-v${VERSION}-x86_64-unknown-linux-musl.tar.gz"
ferris-ci install-tool sccache $URL

# Install cargo-nextest
VERSION="0.9.33"
URL="https://get.nexte.st/${VERSION}/x86_64-unknown-linux-musl.tar.gz"
ferris-ci install-tool cargo-nextest $URL

# Install circleci-junit-fix
VERSION="0.2.2"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/circleci-junit-fix-0.2.2.tar.gz"
ferris-ci install-tool circleci-junit-fix $URL

# Install cargo-machete
VERSION="0.3.1"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/cargo-machete-${VERSION}.tar.gz"
ferris-ci install-tool cargo-machete $URL

# Install cargo-llvm-cov
VERSION="0.4.14"
URL="https://github.com/taiki-e/cargo-llvm-cov/releases/download/v${VERSION}/cargo-llvm-cov-x86_64-unknown-linux-musl.tar.gz"
ferris-ci install-tool cargo-llvm-cov $URL
