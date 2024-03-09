This CLI extracts all commands in a given bash script.

# Usage

* [Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
* [Install Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).
* Run `git clone https://github.com/yutaro-sakamoto/extract-bash-command` to clone this repository,.
* Run `cargo build --release` to build this repository.
* You can have the executable file in `target/release/extract-bash-command`. Copy it to a directory that `$PATH` specifies. (e.g. /usr/bin/)
* Run `extract-bash-command <path-to-bash-script>` to extract all commands in the given bash script.
  * To see the help message, run `extract-bash-command -h` or `extract-bash-command --help`.