#!/bin/bash
set -ex

yum -y update
yum -y install centos-release-scl epel-release
yum -y install scl-utils scl-utils-build
yum -y install  \
   ca-certificates \
   gcc \
   glibc-devel \
   tar \
   zstd \
   devtoolset-9-gcc-c++

scl enable devtoolset-9 "find /opt/rh/devtoolset-9/root/usr -name '*.so' -exec xargs strip -s {} \;"
cd /
chmod -R 777 /opt
chmod 777 /usr/lib64/*.o
tar -cv /opt/rh/devtoolset-9/root/usr/include /opt/rh/devtoolset-9/root/usr/lib | zstd --ultra -22 --long=31 -o /io/devtoolset-9.tar.zst 


