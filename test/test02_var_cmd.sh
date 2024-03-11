#!/bin/bash
ECHO=echo
SED=sed
$ECHO hello
export TEST_VAR=HELLO
echo "$($ECHO $TEST_VAR)"
if [ "hello" = "world" ]; then
    TEST_VAR=WORLD echo $TEST_VAR # echo hello
else
    cat <(echo hello | $SED s/hello/world/g)
fi