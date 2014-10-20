iron-file-server
================

> Simple file server based on [Iron](https://github.com/iron/iron) web framework.

## Building

To build, you'll need the [Rust](http://www.rust-lang.org/) and [Cargo](http://doc.crates.io/) installed. Then navigate to the project root and type:

```
cargo run --release
```

## Usage

Iron file server starts a HTTP server in the directory it is started from, serving all files on their respective relative paths. You can specify the port with `-p` option:

```bash
$> iron-file-server -p 12345
Running simple RUST file server on port 12345
Sering folder /Users/golergka/Projects/rust/iron-file-server
Press Ctrl-C to quit
```
