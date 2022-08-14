#! /bin/bash

find . -name "lib.rs" -exec rustfmt --edition 2021 --check {} \;
find . -name "main.rs" -exec rustfmt --edition 2021 --check {} \;
