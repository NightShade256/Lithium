# Lithium

A simple implementation of Conway's Game of Life written in Rust, targeting WASM.

## Build Instructions

Lithium requires that you have the `wasm32-unknown-unknown` Rust target, `wasm-bindgen` CLI tool and
Python 3 (for the `http.server` module) installed.

To build the WASM binary, execute

```bash
cargo xtask build
```

To serve the web page, execute

```bash
cargo xtask serve
```

To remove the build WASM artifacts, execute

```bash
cargo xtask clean
```

## License

Lithium is licensed under the terms of the Apache 2.0 license.
