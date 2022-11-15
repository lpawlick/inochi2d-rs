//! # inochi2d-rs
//!
//! A pure Rust implementation of [Inochi2D](https://inochi2d.com), the realtime 2D puppet
//! animation framework.

pub mod gl;
mod parser;

pub use parser::{BlendMode, Mask, Meta, Model, Node, Puppet, Transform};
