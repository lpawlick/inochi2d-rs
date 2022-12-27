// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(not(target_arch = "wasm32"))]
use core::num::{NonZeroI32, NonZeroU32};
#[cfg(not(target_arch = "wasm32"))]
use core::ptr::null;
#[cfg(not(target_arch = "wasm32"))]
use std::ffi::CString;

pub const ONE: u32 = 1;
pub const TRIANGLES: u32 = 0x0004;
pub const TRIANGLE_STRIP: u32 = 0x0005;
pub const ALWAYS: u32 = 0x0207;
pub const EQUAL: u32 = 0x0202;
pub const ONE_MINUS_SRC_COLOR: u32 = 0x0301;
pub const ONE_MINUS_SRC_ALPHA: u32 = 0x0303;
pub const DST_ALPHA: u32 = 0x0304;
pub const DST_COLOR: u32 = 0x0306;
pub const STENCIL_TEST: u32 = 0x0B90;
pub const BLEND: u32 = 0x0BE2;
pub const TEXTURE_2D: u32 = 0x0DE1;
pub const UNSIGNED_BYTE: u32 = 0x1401;
pub const UNSIGNED_SHORT: u32 = 0x1403;
pub const FLOAT: u32 = 0x1406;
pub const RGBA: u32 = 0x1908;
pub const KEEP: u32 = 0x1E00;
pub const REPLACE: u32 = 0x1E01;
pub const LINEAR: u32 = 0x2601;
pub const TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const TEXTURE_MIN_FILTER: u32 = 0x2801;
pub const TEXTURE_WRAP_S: u32 = 0x2802;
pub const TEXTURE_WRAP_T: u32 = 0x2803;
pub const CLAMP_TO_EDGE: u32 = 0x812F;
pub const ARRAY_BUFFER: u32 = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: u32 = 0x8893;
pub const STATIC_DRAW: u32 = 0x88E4;
pub const DYNAMIC_DRAW: u32 = 0x88E8;
pub const FRAGMENT_SHADER: u32 = 0x8B30;
pub const VERTEX_SHADER: u32 = 0x8B31;
pub const COMPILE_STATUS: u32 = 0x8B81;
pub const LINK_STATUS: u32 = 0x8B82;
#[cfg(not(target_arch = "wasm32"))]
const INFO_LOG_LENGTH: u32 = 0x8B84;
pub const FRAMEBUFFER_COMPLETE: u32 = 0x8CD5;
pub const COLOR_ATTACHMENT0: u32 = 0x8CE0;
pub const FRAMEBUFFER: u32 = 0x8D40;

pub const STENCIL_BUFFER_BIT: u32 = 0x00000400;
pub const COLOR_BUFFER_BIT: u32 = 0x00004000;

// GL_KHR_debug
#[cfg(feature = "debug")]
pub const DEBUG_SOURCE_APPLICATION: u32 = 0x824A;

// GL_EXT_texture_compression_bptc
pub const COMPRESSED_RGBA_BPTC_UNORM: u32 = 0x8E8C;

// GL_KHR_texture_compression_astc_hdr
pub const COMPRESSED_RGBA_ASTC_4X4: u32 = 0x93B0;
pub const COMPRESSED_RGBA_ASTC_5X4: u32 = 0x93B1;
pub const COMPRESSED_RGBA_ASTC_5X5: u32 = 0x93B2;
pub const COMPRESSED_RGBA_ASTC_6X5: u32 = 0x93B3;
pub const COMPRESSED_RGBA_ASTC_6X6: u32 = 0x93B4;
pub const COMPRESSED_RGBA_ASTC_8X5: u32 = 0x93B5;
pub const COMPRESSED_RGBA_ASTC_8X6: u32 = 0x93B6;
pub const COMPRESSED_RGBA_ASTC_8X8: u32 = 0x93B7;
pub const COMPRESSED_RGBA_ASTC_10X5: u32 = 0x93B8;
pub const COMPRESSED_RGBA_ASTC_10X6: u32 = 0x93B9;
pub const COMPRESSED_RGBA_ASTC_10X8: u32 = 0x93BA;
pub const COMPRESSED_RGBA_ASTC_10X10: u32 = 0x93BB;
pub const COMPRESSED_RGBA_ASTC_12X10: u32 = 0x93BC;
pub const COMPRESSED_RGBA_ASTC_12X12: u32 = 0x93BD;

#[cfg(target_arch = "wasm32")]
pub type NativeProgram = web_sys::WebGlProgram;
#[cfg(target_arch = "wasm32")]
pub type NativeTexture = web_sys::WebGlTexture;
#[cfg(target_arch = "wasm32")]
pub type NativeBuffer = web_sys::WebGlBuffer;
#[cfg(target_arch = "wasm32")]
pub type NativeUniformLocation = web_sys::WebGlUniformLocation;
#[cfg(target_arch = "wasm32")]
pub type NativeFramebuffer = web_sys::WebGlFramebuffer;
#[cfg(target_arch = "wasm32")]
pub type Context = web_sys::WebGlRenderingContext;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, PartialEq)]
pub struct NativeProgram(NonZeroU32);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct NativeShader(NonZeroU32);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, PartialEq)]
pub struct NativeTexture(NonZeroU32);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct NativeBuffer(NonZeroU32);
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeUniformLocation(NonZeroI32);
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct NativeFramebuffer(NonZeroU32);

#[cfg(not(target_arch = "wasm32"))]
#[link(name = "GLESv2")]
extern "C" {
    fn glEnable(cap: u32);
    fn glDisable(cap: u32);
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    fn glClear(mask: u32);
    fn glStencilOp(fail: u32, zfail: u32, zpass: u32);
    fn glStencilFunc(func: u32, ref_: i32, mask: u32);
    fn glStencilMask(mask: u32);
    fn glColorMask(r: bool, g: bool, b: bool, a: bool);
    fn glBlendFunc(sfactor: u32, dfactor: u32);
    fn glEnableVertexAttribArray(index: u32);
    fn glVertexAttribPointer(
        index: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        pointer: u32,
    );
    fn glGenBuffers(n: i32, out: *mut u32);
    fn glBindBuffer(target: u32, buffer: u32);
    fn glBufferData(target: u32, size: isize, data: *const u8, usage: u32);
    fn glBufferSubData(target: u32, offset: i32, size: isize, data: *const u8);
    fn glDrawArrays(mode: u32, first: i32, count: i32);
    fn glDrawElements(mode: u32, count: i32, type_: u32, indices: i32);
    fn glGenTextures(n: i32, out: *mut u32);
    fn glBindTexture(target: u32, tex: u32);
    fn glDeleteTextures(n: i32, textures: *const u32);
    fn glTexImage2D(
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: *const u8,
    );
    fn glCompressedTexImage2D(
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        image_size: i32,
        data: *const u8,
    );
    fn glTexParameteri(target: u32, pname: u32, param: i32);
    fn glGenFramebuffers(n: i32, out: *mut u32);
    fn glBindFramebuffer(target: u32, fbo: u32);
    fn glDeleteFramebuffers(n: i32, framebuffers: *const u32);
    fn glFramebufferTexture2D(
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: u32,
        level: i32,
    );
    fn glCheckFramebufferStatus(target: u32) -> u32;
    fn glCreateShader(type_: u32) -> u32;
    fn glShaderSource(shader: u32, count: i32, string: *const *const u8, length: *const i32);
    fn glCompileShader(shader: u32);
    fn glGetShaderiv(shader: u32, pname: u32, params: *mut i32);
    fn glGetShaderInfoLog(shader: u32, buf_size: i32, length: *mut i32, info_log: *mut u8);
    fn glDeleteShader(shader: u32);
    fn glCreateProgram() -> u32;
    fn glAttachShader(program: u32, shader: u32);
    fn glLinkProgram(program: u32);
    fn glGetProgramiv(program: u32, pname: u32, params: *mut i32);
    fn glGetProgramInfoLog(program: u32, buf_size: i32, length: *mut i32, info_log: *mut u8);
    fn glUseProgram(program: u32);
    fn glDeleteProgram(program: u32);
    fn glGetUniformLocation(program: u32, name: *const u8) -> i32;
    fn glUniform1f(location: i32, v0: f32);
    fn glUniform2f(location: i32, v0: f32, v1: f32);

    // GL_KHR_debug
    #[cfg(feature = "debug")]
    fn glPushDebugGroup(source: u32, id: u32, length: i32, message: *const u8);
    #[cfg(feature = "debug")]
    fn glPopDebugGroup();
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Context;

#[cfg(not(target_arch = "wasm32"))]
impl Context {
    pub fn new() -> Context {
        Context
    }

    pub fn enable(&self, cap: u32) {
        unsafe { glEnable(cap) };
    }

    pub fn disable(&self, cap: u32) {
        unsafe { glDisable(cap) };
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { glViewport(x, y, width, height) };
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe { glClearColor(r, g, b, a) };
    }

    pub fn clear(&self, mask: u32) {
        unsafe { glClear(mask) };
    }

    pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) {
        unsafe { glStencilOp(fail, zfail, zpass) };
    }

    pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32) {
        unsafe { glStencilFunc(func, ref_, mask) };
    }

    pub fn stencil_mask(&self, mask: u32) {
        unsafe { glStencilMask(mask) };
    }

    pub fn color_mask(&self, r: bool, g: bool, b: bool, a: bool) {
        unsafe { glColorMask(r, g, b, a) };
    }

    pub fn blend_func(&self, sfactor: u32, dfactor: u32) {
        unsafe { glBlendFunc(sfactor, dfactor) };
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe { glEnableVertexAttribArray(index) };
    }

    pub fn vertex_attrib_pointer_with_i32(
        &self,
        index: u32,
        size: i32,
        type_: u32,
        normalized: bool,
        stride: i32,
        pointer: u32,
    ) {
        unsafe { glVertexAttribPointer(index, size, type_, normalized, stride, pointer) };
    }

    pub fn create_buffer(&self) -> Option<NativeBuffer> {
        let mut buf = 0u32;
        unsafe { glGenBuffers(1, &mut buf) };
        NonZeroU32::new(buf).map(NativeBuffer)
    }

    pub fn bind_buffer(&self, target: u32, buffer: Option<&NativeBuffer>) {
        let buffer = match buffer {
            None => 0,
            Some(NativeBuffer(buffer)) => buffer.get(),
        };
        unsafe { glBindBuffer(target, buffer) };
    }

    pub fn buffer_data_with_u8_array(&self, target: u32, bytes: &[u8], usage: u32) {
        let size = bytes.len() as isize;
        let data = bytes.as_ptr();
        unsafe { glBufferData(target, size, data, usage) };
    }

    pub fn buffer_sub_data_with_i32_and_u8_array(&self, target: u32, offset: i32, bytes: &[u8]) {
        let size = bytes.len() as isize;
        let data = bytes.as_ptr();
        unsafe { glBufferSubData(target, offset, size, data) };
    }

    pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        unsafe { glDrawArrays(mode, first, count) };
    }

    pub fn draw_elements_with_i32(&self, mode: u32, count: i32, type_: u32, indices: i32) {
        unsafe { glDrawElements(mode, count, type_, indices) };
    }

    pub fn create_texture(&self) -> Option<NativeTexture> {
        let mut tex = 0u32;
        unsafe { glGenTextures(1, &mut tex) };
        NonZeroU32::new(tex).map(NativeTexture)
    }

    pub fn bind_texture(&self, target: u32, texture: Option<&NativeTexture>) {
        let texture = match texture {
            None => 0,
            Some(NativeTexture(texture)) => texture.get(),
        };
        unsafe { glBindTexture(target, texture) };
    }

    pub fn delete_texture(&self, texture: Option<&NativeTexture>) {
        let texture = match texture {
            None => 0,
            Some(NativeTexture(texture)) => texture.get(),
        };
        unsafe { glDeleteTextures(1, &texture) };
    }

    pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        &self,
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) -> Result<(), ()> {
        let pixels = match pixels {
            None => null(),
            Some(pixels) => pixels.as_ptr(),
        };
        unsafe {
            glTexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                pixels,
            )
        };
        Ok(())
    }

    pub fn compressed_tex_image_2d_with_u8_array(
        &self,
        target: u32,
        level: i32,
        internalformat: u32,
        width: i32,
        height: i32,
        border: i32,
        pixels: &[u8],
    ) {
        let image_size = pixels.len() as i32;
        let pixels = pixels.as_ptr();
        unsafe {
            glCompressedTexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                image_size,
                pixels,
            )
        };
    }

    pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) {
        unsafe { glTexParameteri(target, pname, param) };
    }

    pub fn create_framebuffer(&self) -> Option<NativeFramebuffer> {
        let mut fbo = 0u32;
        unsafe { glGenFramebuffers(1, &mut fbo) };
        NonZeroU32::new(fbo).map(NativeFramebuffer)
    }

    pub fn bind_framebuffer(&self, target: u32, fbo: Option<&NativeFramebuffer>) {
        let fbo = match fbo {
            None => 0,
            Some(NativeFramebuffer(fbo)) => fbo.get(),
        };
        unsafe { glBindFramebuffer(target, fbo) };
    }

    pub fn delete_framebuffer(&self, framebuffer: Option<&NativeFramebuffer>) {
        let framebuffer = match framebuffer {
            None => 0,
            Some(NativeFramebuffer(framebuffer)) => framebuffer.get(),
        };
        unsafe { glDeleteFramebuffers(1, &framebuffer) };
    }

    pub fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        textarget: u32,
        texture: Option<&NativeTexture>,
        level: i32,
    ) {
        let texture = match texture {
            None => 0,
            Some(NativeTexture(texture)) => texture.get(),
        };
        unsafe { glFramebufferTexture2D(target, attachment, textarget, texture, level) };
    }

    pub fn check_framebuffer_status(&self, target: u32) -> u32 {
        unsafe { glCheckFramebufferStatus(target) }
    }

    pub fn create_shader(&self, type_: u32) -> Option<NativeShader> {
        let shader = unsafe { glCreateShader(type_) };
        NonZeroU32::new(shader).map(NativeShader)
    }

    pub fn shader_source(&self, shader: &NativeShader, source: &str) {
        let count = 1;
        let length = source.len() as i32;
        let string = source.as_ptr();
        unsafe { glShaderSource(shader.0.get(), count, &string, &length) };
    }

    pub fn compile_shader(&self, shader: &NativeShader) {
        unsafe { glCompileShader(shader.0.get()) };
    }

    pub fn get_shader_parameter(&self, shader: &NativeShader, pname: u32) -> bool {
        let mut status = 0i32;
        unsafe { glGetShaderiv(shader.0.get(), pname, &mut status) };
        status != 0
    }

    pub fn get_shader_info_log(&self, shader: &NativeShader) -> Option<String> {
        let mut length = 0i32;
        unsafe { glGetShaderiv(shader.0.get(), INFO_LOG_LENGTH, &mut length) };
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            unsafe { glGetShaderInfoLog(shader.0.get(), length, &mut length, log.as_mut_ptr()) };
            Some(log)
        } else {
            None
        }
    }

    pub fn delete_shader(&self, shader: Option<&NativeShader>) {
        let shader = match shader {
            None => 0,
            Some(NativeShader(shader)) => shader.get(),
        };
        unsafe { glDeleteShader(shader) };
    }

    pub fn create_program(&self) -> Option<NativeProgram> {
        let program = unsafe { glCreateProgram() };
        NonZeroU32::new(program).map(NativeProgram)
    }

    pub fn attach_shader(&self, program: &NativeProgram, shader: &NativeShader) {
        unsafe { glAttachShader(program.0.get(), shader.0.get()) };
    }

    pub fn link_program(&self, program: &NativeProgram) {
        unsafe { glLinkProgram(program.0.get()) };
    }

    pub fn get_program_parameter(&self, program: &NativeProgram, pname: u32) -> bool {
        let mut status = 0i32;
        unsafe { glGetProgramiv(program.0.get(), pname, &mut status) };
        status != 0
    }

    pub fn get_program_info_log(&self, program: &NativeProgram) -> Option<String> {
        let mut length = 0i32;
        unsafe { glGetProgramiv(program.0.get(), INFO_LOG_LENGTH, &mut length) };
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            unsafe { glGetProgramInfoLog(program.0.get(), length, &mut length, log.as_mut_ptr()) };
            Some(log)
        } else {
            None
        }
    }

    pub fn use_program(&self, program: Option<&NativeProgram>) {
        let program = match program {
            None => 0,
            Some(NativeProgram(program)) => program.get(),
        };
        unsafe { glUseProgram(program) };
    }

    pub fn delete_program(&self, program: Option<&NativeProgram>) {
        let program = match program {
            None => 0,
            Some(NativeProgram(program)) => program.get(),
        };
        unsafe { glDeleteProgram(program) };
    }

    pub fn get_uniform_location(
        &self,
        program: &NativeProgram,
        name: &str,
    ) -> Option<NativeUniformLocation> {
        let name = CString::new(name).unwrap();
        let location = unsafe { glGetUniformLocation(program.0.get(), name.as_ptr() as *const _) };
        NonZeroI32::new(location).map(NativeUniformLocation)
    }

    pub fn uniform1f(&self, location: Option<&NativeUniformLocation>, v0: f32) {
        let location = match location {
            None => 0,
            Some(NativeUniformLocation(location)) => location.get(),
        };
        unsafe { glUniform1f(location, v0) };
    }

    pub fn uniform2f(&self, location: Option<&NativeUniformLocation>, v0: f32, v1: f32) {
        let location = match location {
            None => 0,
            Some(NativeUniformLocation(location)) => location.get(),
        };
        unsafe { glUniform2f(location, v0, v1) };
    }

    // GL_KHR_debug

    #[cfg(feature = "debug")]
    pub fn push_debug_group(&self, source: u32, id: u32, message: &str) {
        unsafe { glPushDebugGroup(source, id, message.len() as i32, message.as_ptr()) };
    }

    #[cfg(feature = "debug")]
    pub fn pop_debug_group(&self) {
        unsafe { glPopDebugGroup() };
    }
}
