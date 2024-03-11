#!/bin/bash

cargo build
CMD='target/debug/extract-bash-command'

diff <($CMD test/test01.sh) test/expected01.txt

# a test for no existing file
if $CMD test/a_file_does_not_exist_908yafa98.txt 2> /dev/null; then
    exit 1
fi

# test --var-cmd
diff <($CMD --var-cmd test/test02_var_cmd.sh) test/expected02_var_cmd.txt
