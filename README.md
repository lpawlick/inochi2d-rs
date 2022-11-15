# inochi2d-rs

A pure Rust implementation of [Inochi2D](https://inochi2d.com), the realtime 2D
puppet animation framework.

This is currently done using [nom](https://crates.io/crates/nom) and
[serde\_json](https://crates.io/crates/serde_json) for the parsing, as well as
[GLFW](https://www.glfw.org) and [glow](https://crates.io/crates/glow) for the
rendering, using OpenGL ES 2.0.

## TODO

- <input type="checkbox" disabled="" checked=""/> Rendering (at least for those two models)
- <input type="checkbox" disabled=""/> Physics
- <input type="checkbox" disabled=""/> Animations
- <input type="checkbox" disabled=""/> Face tracking

## Screenshots

<picture width="640"><source type="image/avif" srcset="https://linkmauve.fr/dev/inochi2d/aka.avif"/><img width="640" src="https://linkmauve.fr/dev/inochi2d/aka.png"/></picture>
