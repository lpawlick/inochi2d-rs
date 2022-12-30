//! # inochi2d-rs
//!
//! A pure Rust implementation of [Inochi2D](https://inochi2d.com), the realtime 2D puppet
//! animation framework.

pub mod gl;
pub mod glow;
mod parser;
mod tga;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use parser::{BlendMode, Mask, Meta, Model, Node, Puppet, Texture, TextureReceiver, Transform};
