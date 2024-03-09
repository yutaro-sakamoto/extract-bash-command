#!/bin/bash
echo hello
export TEST_VAR=HELLO
echo "$(echo $TEST_VAR)"
if [ "hello" = "world" ]; then
    TEST_VAR=WORLD echo $TEST_VAR # echo hello
else
    cat <(echo hello | sed s/hello/world/g)
fi
