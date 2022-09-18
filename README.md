# ポテート (potēto)

An implementation of the single-page RPG [Potato by Oliver Darkshire](https://twitter.com/deathbybadger/status/1567425842526945280) written for the [WASM-4](https://wasm4.org) fantasy console.

![Game screen](https://github.com/michaelenger/poteto/raw/main/screenshot1.png)
![Roll result screen](https://github.com/michaelenger/poteto/raw/main/screenshot2.png)

[Give it a try!](https://michaelenger.github.io/poteto/)

## Requirements

* [Rust](https://www.rust-lang.org/)
* [WASM-4](https://wasm4.org/)

## Build

Build the cart by running:

```shell
cargo build --release
```

Then run it with:

```shell
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

## Release

If you want to "release" the game by creating a runnable website you should first run it through the WASM optimiser provided by [binaryen](https://github.com/WebAssembly/binaryen):

```shell
wasm-opt target/wasm32-unknown-unknown/release/cart.wasm -o poteto.wasm -Oz --strip-dwarf --strip-producers --zero-filled-memory
```

This spits out a `poteto.wasm` file which can be bundled with the WASM-4 runtime into a website:

```shell
w4 bundle poteto.wasm --title "ポテート" --html index.html
```

The resulting `index.html` can be run as-is, even offline.
