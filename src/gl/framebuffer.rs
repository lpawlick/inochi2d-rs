// Copyright (c) 2023 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::texture::Texture;
use crate::glow;

#[derive(Clone)]
pub struct Framebuffer<'a> {
    gl: &'a glow::Context,
    fbo: glow::NativeFramebuffer,
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        let gl = self.gl;
        gl.delete_framebuffer(Some(&self.fbo));
    }
}

impl<'a> Framebuffer<'a> {
    pub fn new(gl: &'a glow::Context) -> Framebuffer<'a> {
        let fbo = gl.create_framebuffer().unwrap();
        Framebuffer { gl, fbo }
    }

    pub fn attach_texture(&self, texture: &Texture) {
        let gl = self.gl;
        self.bind();
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::TEXTURE_2D,
            Some(&texture.texture),
            0,
        );
        assert_eq!(
            gl.check_framebuffer_status(glow::FRAMEBUFFER),
            glow::FRAMEBUFFER_COMPLETE
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
    }

    pub fn bind(&self) {
        let gl = self.gl;
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(&self.fbo));
    }
}
