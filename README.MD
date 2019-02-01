# Demo WebServer App

This is a demo WebApp written in Rust

- Frontend: written in Rust and compilled to WebAssembly
- Backend TBD

## Prerequisite

Note: _Rust nightly is mandatory for webassembly_

Have rust nightly installed:

```sh
rustup install nightly
```

Switch to nightly version:

```sh
rustup default nightly
```

Add the native WebAssembly Rust compilation target

```sh
rustup target add wasm32-unknown-unknown
```

Install cargo web:

```sh
cargo install cargo-web
```

Check installatio and rust version with:

```sh
rustup show
```

## Build

## Run

Run webserver locally

```sh
cargo web start -p frontend --target wasm32-unknown-unknown
```

### TODO
