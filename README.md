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
% git clone https://git.linkmauve.fr/inochi2d.git
```
Build:
```shell
% cargo build --release
```
Run:
```shell
% cargo run --release <puppet.inp>
```

You can find two example puppets in [Inochi2D’s example models](https://github.com/Inochi2D/example-models).

## TODO

- <input type="checkbox" disabled="" checked=""/> Rendering (at least for those two models)
- <input type="checkbox" disabled=""/> Physics
- <input type="checkbox" disabled=""/> Animations
- <input type="checkbox" disabled=""/> Face tracking

## Screenshots

<picture width="640"><source type="image/avif" srcset="https://linkmauve.fr/dev/inochi2d/aka.avif"/><img width="640" src="https://linkmauve.fr/dev/inochi2d/aka.png"/></picture>
