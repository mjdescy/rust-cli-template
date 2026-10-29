# Rust CLI Template

This project is meant to be a template for simple CLI applications. It provides the following features:

- command-line arguments (clap derive)
- subcommands (`hello` as an example)
- `-v/--verbose` flag (repeatable)
- `-V/--version` flag
- `help` subcommand and `-h/--help` flag

## Usage

```sh
cargo run -- --help
cargo run -- --version
cargo run -- help hello
cargo run -- -v hello World
```