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

## Testing commands

Run the CLI integration tests with:

```sh
cargo test
```

`tests/cli.rs` is an example to follow when adding commands. Each test runs the compiled CLI with arguments and checks its exit status and output. For a new command, add tests for its successful behavior, its `--help` output, and missing or invalid arguments where applicable.