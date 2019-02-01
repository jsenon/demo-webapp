# Demo WebServer App

**Currently in DRAFT**

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

Run frontend webserver locally

```sh
cargo web start -p frontend --target wasm32-unknown-unknown
```

Run backend webserver locally

```sh
cargo run -p backend 
```

## Usage

### Frontend

Open web browser on http://localhost:8000

### Backend

Get version info

```sh
 curl -X GET http://localhost:3000/

```

Create User

```sh
 curl -X POST http://localhost:3000/user -d '{"user": "toto"}'
```

### TODO

- [ ] Link frontend to backend
- [ ] DockerFile
- [ ] Add opentracing func