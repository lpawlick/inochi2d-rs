# inochi2d-rs

[![crates.io](https://img.shields.io/crates/v/inochi2d.svg)](https://crates.io/crates/inochi2d)
[![docs.rs](https://docs.rs/inochi2d/badge.svg)](https://docs.rs/inochi2d)
[![License](https://img.shields.io/crates/l/inochi2d.svg)](https://www.mozilla.org/MPL/2.0/)
[![Join the XMPP chat at inochi2d@muc.linkmauve.fr](https://linkmauve.fr/badge/inochi2d@muc.linkmauve.fr)](https://converse.linkmauve.fr/inochi2d@muc.linkmauve.fr)

A pure Rust implementation of [Inochi2D](https://inochi2d.com), the realtime 2D
puppet animation framework.

This is currently done using [serde\_json](https://crates.io/crates/serde_json)
for parsing, as well as [GLFW](https://www.glfw.org) and
[glow](https://crates.io/crates/glow) for the rendering, using OpenGL ES 2.0.

## How to use

Clone the repository:
```shell
git clone https://git.linkmauve.fr/inochi2d.git
```
Build:
```shell
cargo build --release
```
Run:
```shell
cargo run --release <puppet.inp>
```

You can find two example puppets in [Inochi2D’s example models](https://github.com/Inochi2D/example-models).

## WebGL support using WebAssembly

You will need to enable the wasm32-unknown-unknown target (for instance using rustup):

```shell
rustup target add wasm32-unknown-unknown
```

Then install wasm-bindgen (also available in ArchLinux):

```shell
cargo install wasm-bindgen
```

Then you can build the wasm file and its assorted helper JS file:

```shell
cargo rustc --lib --crate-type cdylib  
cargo build --target=wasm32-unknown-unknown --lib --release --no-default-features
wasm-bindgen target/wasm32-unknown-unknown/release/inochi2d.wasm --target=web --out-dir=pkg
```

Everything you need will be in the pkg/ directory. You can now import the JS file from your own scripts, check the example /examples/wasm_example.js included here.

## Status

- <input type="checkbox" disabled="" checked=""/> Rendering (at least for those two models)
    - Tested platforms:
        - <input type="checkbox" disabled="" checked=""/> Thinkpad x280 running ArchLinux
        - <input type="checkbox" disabled="" checked=""/> PinePhone running ArchLinuxARM
        - <input type="checkbox" disabled="" checked=""/> WebGL in Firefox
- <input type="checkbox" disabled="" checked=""/> Animations (incomplete)
- <input type="checkbox" disabled="" /> Physics
- <input type="checkbox" disabled="" /> Face tracking

## Screenshots

<picture width="640"><source type="image/avif" srcset="https://linkmauve.fr/dev/inochi2d/aka.avif"/><img width="640" src="https://linkmauve.fr/dev/inochi2d/aka.png"/></picture>