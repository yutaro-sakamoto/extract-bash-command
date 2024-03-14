This CLI extracts all commands in a given bash script.

# Usage

Run `extract-bash-command <paths to bash scripts>` to extract all commands in the given bash script.
To see the help message, run `extract-bash-command -h` or `extract-bash-command --help`.

# Installation

## Manual installation

* [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
* [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).
* Run `git clone https://github.com/yutaro-sakamoto/extract-bash-command` to clone this repository,.
* Run `cargo build --release` to build this repository.
* You can have the executable file in `target/release/extract-bash-command`. Copy it to a directory that `$PATH` specifies. (e.g. /usr/bin/)

## Using pre-built binary 

Visit the release page and download the binary for your environment.

# Example

sample.sh  
```bash
#!/bin/bash
echo hello
export TEST_VAR=HELLO
echo "$(echo $TEST_VAR)"
if [ "hello" = "world" ]; then
    TEST_VAR=WORLD echo $TEST_VAR # echo world
else
    cat <(echo hello | sed s/hello/world/g)
fi
```

The command
```
$ extract-bash-command sample.sh
```

The output
```
echo 2 1
echo 4 1
echo 4 9
echo 6 20
cat 8 5
echo 8 11
sed 8 24
```
