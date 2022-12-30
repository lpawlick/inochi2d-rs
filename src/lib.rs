// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
