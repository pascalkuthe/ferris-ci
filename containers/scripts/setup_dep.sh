#!/bin/sh

set -e

apk add --no-cache ca-certificates

# Install cargo-machete
VERSION="0.3.1"
URL="https://openva.fra1.cdn.digitaloceanspaces.com/cargo-machete-${VERSION}.tar.gz"
ferris-ci install-tool cargo-machete $URL

rm /usr/bin/ferris-ci
