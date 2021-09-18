# Version

The blazing fast way to check what version of grep you're using, written in Rust.

This is a Rust rewrite of the [`version`](https://github.com/bit101/version) tool written by [Keith Peters](http://www.bit-101.com/).
I wanted to write a complete project in Rust that I would actually find useful, and I thought it would be funny to rewrite such a simple tool in Rust.

## Usage

```shell
version <tool name>
```

For example:

```text
version python3
Python 3.9.5
```

## Installation

If you don't have the Rust toolchain, you can install it using [rustup](https://rustup.rs), then run:

```shell
cargo install --git "https://github.com/Hasnep/version.git"
```

## Licencing

This project is released under the MIT Licence found in the [licence file](LICENCE).
It also uses code from Keith Peters' project [`version`](https://github.com/bit101/version), licenced under the MIT Licence, which can be found the [project's repository](https://github.com/bit101/version/blob/master/LICENSE) or in the file [`LICENCE-bit101`](LICENCE-bit101).
