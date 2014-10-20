iron-file-server
================

> Simple file server based on [Iron](https://github.com/iron/iron) web framework.

## Perfomance

The server can habdle up to 10k requests per second on a regular machine:

```bash
wrk -t12 -c900 -d10s http://127.0.0.1:3000/Cargo.toml
Running 10s test @ http://127.0.0.1:3000/Cargo.toml
  12 threads and 900 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   138.00ms  320.29ms   1.86s    91.06%
    Req/Sec     1.24k     1.55k   10.61k    89.79%
  104911 requests in 10.02s, 43.72MB read
  Socket errors: connect 0, read 881, write 0, timeout 1030
Requests/sec:  10473.83
Transfer/sec:      4.37MB
```

## Building

To build, you'll need the [Rust](http://www.rust-lang.org/) and [Cargo](http://doc.crates.io/) installed. Then navigate to the project root and type:

```
cargo run --release
```

## Usage

Iron file server starts a HTTP server in the directory it is started from, serving all files on their respective relative paths. You can specify the port with `-p` option:

```bash
$> iron-file-server -p 12345
Running simple Iron file server on port 12345
Sharing folder /path/to/your/project/folder
Press Ctrl-C to quit
```
