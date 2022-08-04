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

# install msvc crt
xwin_version="0.2.5"
xwin_prefix="xwin-$xwin_version-x86_64-unknown-linux-musl"
ferris-ci install-tool xwin https://github.com/Jake-Shadle/xwin/releases/download/$xwin_version/$xwin_prefix.tar.gz
xwin --accept-license splat --output /xwin
rm -rf .xwin-cache /bin/xwin

# add rust target for windows-msvc
rustup target add x86_64-pc-windows-msvc

# jerry rig llvm
ln -s llvm-ar /usr/bin/llvm-dlltool
rm /usr/bin/llvm-config # will be replaced with script
