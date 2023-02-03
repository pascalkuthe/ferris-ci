#!/bin/bash
set -eux

# install wine
VERSION="7.14"
ferris-ci download "wine-${VERSION}-x86_64-unknown-linux-gnu-OFF.tar.zst" --decompress
mv WINE/bin/* /usr/bin/
mv WINE/lib64/* /usr/lib64/
ln -s /usr/lib64/wine /usr/lib/wine
ln -s /usr/lib64/LLVM* /usr/lib/
mv WINE/share/* /usr/share/
rm -rf WINE
wine64 wineboot --init

# install msvc crt
VERSION=0.2.10
URL="https://openva.fra1.cdn.digitaloceanspaces.com/cargo-xwin-${VERSION}.tar.gz"
ferris-ci install-tool xwin $URL
xwin --accept-license --manifest-version=17 splat  --output /xwin
rm -rf .xwin-cache /bin/xwin

# add rust target for windows-msvc
rustup target add x86_64-pc-windows-msvc

# jerry rig llvm
rm /usr/bin/llvm-config # will be replaced with script
