#!/bin/bash
set -ex
rpm -ivh https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
rpm -ql epel-release
microdnf -y update
microdnf -y install --nodocs \
   ca-certificates \
   gcc \
   glibc-devel \
   tar \
   zstd
microdnf -y install --nodocs --enablerepo="ubi-server-rhscl-7-rpms" rh-python38-python rh-python38-python-numpy
find /opt/rh/rh-python38/root/usr -name '*.so' | xargs strip -s
rm -rf /opt/rh/rh-python38/root/lib64/python*/ensurepip
rm -rf /opt/rh/rh-python38/root/lib64/python*/idlelib
rm -rf /opt/rh/rh-python38/root/lib64/python*/distutils/command
rm -rf /opt/rh/rh-python38/root/lib64/python*/lib2to2
rm -rf /opt/rh/rh-python38/root/lib64/python*/lib2to3
rm -rf /opt/rh/rh-python38/root/lib64/python*/sqlite3
rm -rf /opt/rh/rh-python38/root/lib64/python*/xml
rm -rf /opt/rh/rh-python38/root/lib64/python*/dom
rm -rf /opt/rh/rh-python38/root/lib64/python*/email
# rm -rf /opt/rh/rh-python38/root/lib64/python*/asyncio
rm -rf /opt/rh/rh-python38/root/lib64/python*/http
rm -rf /opt/rh/rh-python38/root/lib64/python*/encoding
rm -rf /opt/rh/rh-python38/root/lib64/python*/pydoc*
rm -rf /opt/rh/rh-python38/root/lib64/python*/lib-dynload/_ssl.*-linux-gnu.so
# rm /opt/rh/rh-python38/root/usr/lib64/python3.8/lib-dynload/_asyncio.cpython-38-x86_64-linux-gnu.so
rm -rf /opt/rh/rh-python38/root/lib64/python*/*/__pycache__
rm -rf /opt/rh/rh-python38/root/lib64/python*/__pycache__
rm -rf /opt/rh/rh-python38/root/lib64/python*/**/__pycache__
# rm -rf /opt/rh/rh-python38/root/lib64/python*/asyncio
rm -rf /opt/rh/rh-python38/root/usr/lib/python3.8/site-packages/**/__pycache__
rm -rf /opt/rh/rh-python38/root/usr/lib/python3.8/site-packages/*/*/__pycache__
rm -rf /opt/rh/rh-python38/root/usr/share/man
rm -rf /opt/rh/rh-python38/root/usr/local/share/man
mv /opt/rh/rh-python38/root/usr/share/locale/en_US/ /tmp_
rm -rf /opt/rh/rh-python38/root/usr/share/locale
mkdir -p /opt/rh/rh-python38/root/usr/share/locale
mv /tmp_ /opt/rh/rh-python38/root/usr/share/locale/en_US
rm -rf /opt/rh/rh-python38/**.rst
rm -rf /opt/rh/rh-python38/root/usr/share/doc
rm -rf /opt/rh/rh-python38/root/usr/share/doc
rm -rf /opt/rh/rh-python38/root/usr/share/doc
rm -rf /opt/rh/rh-python38/root/usr/share/doc
cd /opt/rh
chmod -R 777 /opt/rh 
tar -cv rh-python38 | zstd --ultra -22 --long=31 -o /io/rh-python38.tar.zst 
