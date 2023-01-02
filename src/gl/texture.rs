// Copyright (c) 2023 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::glow;

#[derive(Clone)]
pub struct Texture<'a> {
    gl: &'a glow::Context,
    pub texture: glow::NativeTexture,
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        let gl = self.gl;
        gl.delete_texture(Some(&self.texture));
    }
}

impl<'a> Texture<'a> {
    pub fn from_data(
        gl: &'a glow::Context,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
    ) -> Result<Texture<'a>, String> {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(&texture));
        gl.tex_parameteri(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameteri(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        if data.is_none() {
            gl.tex_parameteri(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
        }
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            data,
        )
        .map_err(|_| String::from("Empty texture allocation failed"))?;
        Ok(Texture { gl, texture })
    }

    pub fn bind(&self) {
        let gl = self.gl;
        gl.bind_texture(glow::TEXTURE_2D, Some(&self.texture));
    }

    pub fn resize(&self, width: i32, height: i32) {
        let gl = self.gl;
        self.bind();
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width as i32,
            height as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            None,
        )
        .unwrap();
    }
}
