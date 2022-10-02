#!/bin/bash
source scl_source enable devtoolset-9
cd /io
ferris-ci "$@"
# exec "$@"
