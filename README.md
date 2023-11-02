<div align="center">

<img src="./.github/logo.png" height="150px">

# Lit

*A minimalism cross-platform git-like version control system written in Rust*

![](https://github.com/muqiuhan/lit/actions/workflows/ci.yaml/badge.svg) 

🚧 Working In Progress.

</div>

## Build and Run
- `make install`
- `make uninstall`

[Makefile](./Makefile):
```makefile
.PHONY: build install uninstall build.release fmt check fix test

build :
	@cargo build

install : build.release
	@cargo install --path .

uninstall :
	@cargo uninstall

build.release :
	@cargo build --release

fmt:
	@cargo fmt

check:
	@cargo clippy

fix:
	@cargo clippy --fix --allow-staged

test:
	@cargo test -- --test-threads=1
```

## Usage
```
USAGE:
    lit <SUBCOMMAND> [FLAGS] [ARGS]

SUBCOMMAND:
    version               Display version information about lit
    help                  Display help information about lit
    add                   Add file contents to the index
    init                  Create an empty lit repository or reinitialize an existing one
    log                   Show commit logs
    rm                    Remove files from the working tree and from the index
    tagging               Create, list, delete or verify a tag object signed with GPG
    status                Show the working tree status
    cat-file              Provide content or type and size information for repository objects
    check-ignore          Debug gitignore / exclude files
    checkout              Switch branches or restore working tree files
    commit                Record changes to the repository
    hash-object           Compute object ID and optionally create an object from a file
    ls-files              Show information about files in the index and the working tree
    ls-tree               List the contents of a tree object
    rev-parse             Pick out and massage parameters
    show-ref              List references in a local repository
```

## Dependencies

| Name                                              | License            | Description                                                                      |
| ------------------------------------------------- | ------------------ | -------------------------------------------------------------------------------- |
| [structopt](https://github.com/TeXitoi/structopt) | Apache 2.0 and MIT | Parse command line arguments by defining a struct.                               |
| [rust-ini](https://github.com/zonyitoo/rust-ini)  | MIT                | INI file parser in Rust                                                          |
| [colog](https://github.com/muqiuhan/rust-colog)   | LGPL 3.0           | A simple color-coded logging implementation for the standard rust logging system |
| [log](https://github.com/rust-lang/log)           | Apache 2.0 and MIT | Logging implementation for Rust                                                  |


## [LICENSE](./LICENSE)

Copyright (C) 2023 Muqiu Han

This library is free software; you can redistribute it and/or
modify it under the terms of the GNU Library General Public
License as published by the Free Software Foundation; either
version 2 of the License, or (at your option) any later version.

This library is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Library General Public License for more details.

You should have received a copy of the GNU Library General Public
License along with this library; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA