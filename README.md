# jetbra-rust

The network filter for Java programs.

## Build

```bash
cargo build --release
```

## Usage

CLI:

```bash
./target/release/jetbra   
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

GUI:

```bash
./target/release/jetbra_egui
```

## TODO

- [ ] Menu
    - [ ] Help
    - [ ] About
