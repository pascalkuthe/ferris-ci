#!/bin/bash
source scl_source enable devtoolset-9
source scl_source enable llvm-toolset-7.0
cd /io
# ferris-ci "$@"
exec "$@"
