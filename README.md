# oWASM Boilerplate

## Prerequisite

Install

- Rust and Cargo - https://doc.rust-lang.org/cargo/getting-started/installation.html
- Wasm-pack - https://rustwasm.github.io/wasm-pack/installer/

## Test

```
$ cargo test -- --nocapture
```

## Build

```
$ wasm-pack build
$ ls pkg/awesome_oracle_bg.wasm
```

## End-to-End Test

```
$ cargo run --package bin -- pkg/awesome_oracle_bg.wasm
```
