#!/bin/bash
set -ex
llvm_version=14
# sudo apt-key add llvm-snapshot.gpg.key
# sudo add-apt-repository "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-$llvm_version main"
sudo apt-get update
sudo apt-get install clang-$llvm_version llvm-$llvm_version
