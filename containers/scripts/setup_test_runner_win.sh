#!/bin/bash
set -eux

# install wine
VERSION="7.14"
ferris-ci download "wine-${VERSION}-x86_64-unknown-linux-gnu-OFF.tar.zst" --decompress
mv WINE/bin/* /usr/bin/
mv WINE/lib64/* /usr/lib/
mv WINE/share/* /usr/share/
rm -rf WINE
wine64 wineboot --init

# Install rustfmt
VERSION="1.5.1"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/rustfmt-${VERSION}.tar.gz"
ferris-ci install-tool rustfmt $URL
