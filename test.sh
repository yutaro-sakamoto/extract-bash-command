#!/bin/bash

diff <(cargo run -- test/test01.sh) test/expected01.txt
if cargo run -- test/a_file_does_not_exist_908yafa98.txt; then
    exit 1
fi
