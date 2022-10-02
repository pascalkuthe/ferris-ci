#!/usr/bin/env bash
set -ex

GCC=7.5.0

curl https://ftp.gnu.org/gnu/gcc/gcc-$GCC/gcc-$GCC.tar.xz | xzcat | tar xf -
cd gcc-$GCC

sed -i'' 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites

./contrib/download_prerequisites
mkdir ../gcc-build
mkdir -p /opt/gcc
cd ../gcc-build
../gcc-$GCC/configure \
    --prefix=/newroot \
    --enable-languages=c,c++ \
    --disable-gnu-unique-object 
make -j$(nproc)
make install
ln -s gcc /newroot/bin/cc

cd ..
rm -rf gcc-build
rm -rf gcc-$GCC

ln /newroot/lib/*.{a,so} -rst /newroot/lib/gcc/x86_64-pc-linux-gnu/$GCC/32/
