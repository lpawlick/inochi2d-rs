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

use std::collections::HashMap;
use parser::Param;
pub use parser::{
    Anim, BlendMode, Mask, Meta, Model, Node, Puppet, Texture, TextureReceiver, Transform,
};

pub struct ParamValues<'a> {
    params: &'a [Param],
    values: Vec<[f32; 2]>,
    index_map: HashMap<String, usize>,
}

impl<'a> ParamValues<'a> {
    pub fn new(params: &'a [Param]) -> ParamValues {
        let mut values = Vec::with_capacity(params.len());
        let mut index_map = HashMap::new();
        for (i, param) in params.iter().enumerate() {
            let x = (param.defaults[0] - param.min[0]) / (param.max[0] - param.min[0]);
            let y = (param.defaults[1] - param.min[1]) / (param.max[1] - param.min[1]);
            values.push([x, y]);
            index_map.insert(param.name.clone(), i);
        }
        ParamValues { params, values, index_map }
    }

    pub fn set(&mut self, name: &str, value: [f32; 2]) {
        if let Some(i) = self.index_map.get(name) {
            self.values[*i] = value;
        }
    }

    pub fn iter(&'a self) -> IterParamValues<'a> {
        IterParamValues {
            params: self.params,
            values: &self.values,
            cur: 0,
        }
    }
}

pub struct IterParamValues<'a> {
    params: &'a [Param],
    values: &'a [[f32; 2]],
    cur: usize,
}

impl<'a> Iterator for IterParamValues<'a> {
    type Item = (&'a Param, [f32; 2]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.params.len() {
            let param = &self.params[self.cur];
            let value = self.values[self.cur];
            self.cur += 1;
            Some((param, value))
        } else {
            None
        }
    }
}
