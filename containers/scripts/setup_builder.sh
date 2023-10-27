#!/bin/bash
set -eux

# Install Basic packes

yum install centos-release-scl -y
yum -y update
yum -y install \
	ca-certificates \
	openssh-clients \
	tar \
	gzip \
	glibc-devel
# remove caches
yum clean all
rm -rf /var/cache/yum
rpm -ev lua rpm rpm-libs yum python rpm-build-libs rpm-python libxml2-python yum-utils python-iniparse python-gobject-base yum-metadata-parser python-chardet python-kitchen python-urlgrabber pygpgme dbus-python python-pycurl pyxattr pyliblzma pyxattr yum-plugin-fastestmirror yum-plugin-ovl pyxattr vim-minimal

# we just need the crt files from gcc so we can remove the (large) gcc/ld binaries
# rm /usr/bin/cc
# rm /usr/bin/cpp
# rm /usr/bin/gcc
# rm /usr/bin/c89
# rm /usr/bin/c99
# rm /usr/bin/gcc*
rm /usr/bin/ld*
rm -rf /usr/libexec
rm -rf /usr/share/doc
rm -rf /usr/share/man

rm /bin/awk
rm /bin/sed
# rm /bin/gcov*
# rm /bin/*gcc
rm /bin/ar
rm -rf /usr/share/man
rm -rf /usr/share/zsh
rm -rf /usr/lib64/python2.7
rm -rf /var/lib/rpm
# rm /bin/rpm*
# rm -rf /usr/lib64/libdnf*
# rm -rf /usr/lib64/librpm*


# install include paths and libs from devtoolset-9
cd /
ferris-ci download --decompress devtoolset-9.tar.zst

# install llvm and use clang instead of gcc
VERSION="16.0.6"
ferris-ci download llvm-${VERSION}-x86_64-unknown-linux-gnu-FULL.tar.zst --decompress
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
	ferris-ci download "llvm-${VERSION}-${CROSS_TARGET}.tar.zst" --decompress
    mv /LLVM/bin/llvm-config* /usr/bin/
fi
ln -s llvm-ar /usr/bin/llvm-dlltool
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
ferris-ci install-rust "${RUST_VERSION}"

# Install sccache
VERSION="0.3.0"
URL="https://github.com/mozilla/sccache/releases/download/v${VERSION}/sccache-v${VERSION}-x86_64-unknown-linux-musl.tar.gz"
ferris-ci install-tool sccache $URL

# Install cargo-nextest 
VERSION="0.9.44"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/cargo-nextest-${VERSION}.tar.gz"
ferris-ci install-tool cargo-nextest $URL
