# Sarosa

## Installation

Requirements:

* install Cap'n Proto. Follow the installation guide [here](https://capnproto.org/install.html)
* install [Rust](https://doc.rust-lang.org/nightly/book/nightly-rust.html)
(the nighlty version)

In order to build the application run the following command in your terminal:

```bash
cargo build --release
```

## Run the application

To run sarosa simply write:
```
cargo run
```

To see a list of options with cargo
```
cargo run -- --help
```

## Options

Sarosa comes with a set of predefined options:
```
Sarosa client.

Usage:
  sarosa [--host <host> --port <port>]
  sarosa --offline
  sarosa (-h | --help)
  sarosa --version

Options:
  -h --help         Show this screen.
  --version         Show version.
  --offline         Run a self-hosted offline server.
  --port <port>     Server port     [default: 7777].
  --host <host>     Server Hostname [default: localhost].
```

## Submitting a bug

If you encounter a bug, open an issue here and specify the
output given by running:

    cargo run -- --version
