# ポテート (potēto)

An implementation of the single-page RPG [Potato by Oliver Darkshire](https://twitter.com/deathbybadger/status/1567425842526945280) written for the [WASM-4](https://wasm4.org) fantasy console.

## Requirements

* [Rust](https://www.rust-lang.org/)
* [WASM-4](https://wasm4.org/)

## Building

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```