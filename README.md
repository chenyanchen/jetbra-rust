# jetbra-rust

## What is it?

A CLI tool to activate JetBrains products.

## Why use it?

Why not?

## How to use it?

Step 1: Open the JetBrains product you want to activate.

Step 2: Run the command `jetbra install` in the terminal.
Download [here](https://github.com/chenyanchen/jetbra-rust/releases).

Step 3: Restart the JetBrains product.

## How does it work?

1. Base on the [network filter](https://gitee.com/ja-netfilter/ja-netfilter) for Java programs.
2. Fill the activation code.

## How to build it?

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started)

```bash
cargo build --release
```

### Usage

#### CLI

```bash
./jetbra
A CLI tool to activate JetBrains products

Usage: jetbra [COMMAND]

Commands:
  list       List all supported applications
  install    Install jetbra for current user
  uninstall  Uninstall jetbra for current user
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

#### GUI:

The GUI version for macOS is ready.

```bash
./target/release/jetbra_egui
```

## TODO

- [ ] Support Windows, Linux. (Don't have a Windows or Linux machine, if you know where the JetBrains products store the
  activation code, please tell me.)
