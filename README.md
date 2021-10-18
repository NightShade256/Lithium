# Lithium

A simple implementation of Conway's Game of Life written in Rust, targeting WASM.

## Build Instructions

The `wasm32-unknown-unknown` Rust target and the `wasm-bindgen` CLI tool is required to build Lithium from source.
Python 3 (the `http.server` module to be specific) is required to serve the web page.

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
