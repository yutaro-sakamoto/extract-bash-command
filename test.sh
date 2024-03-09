#!/bin/bash

diff <(cargo run -- test/test01.sh) test/expected01.txt
