// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::glow;

pub enum Vbo<'a, T: Copy> {
    Buffering(&'a glow::Context, Vec<T>),
    Uploaded(glow::NativeBuffer),
}

impl<'a, T: Copy> Vbo<'a, T> {
    pub fn new(gl: &'a glow::Context) -> Vbo<'a, T> {
        Vbo::Buffering(gl, Vec::new())
    }

    pub fn from(gl: &'a glow::Context, vec: Vec<T>) -> Vbo<'a, T> {
        Vbo::Buffering(gl, vec)
    }

    pub fn len(&self) -> usize {
        match self {
            Vbo::Buffering(_, vec) => vec.len(),
            _ => panic!("Vbo must not be uploaded yet!"),
        }
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        match self {
            Vbo::Buffering(_, vec) => vec.extend_from_slice(other),
            _ => panic!("Vbo must not be uploaded yet!"),
        }
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, other: I) {
        match self {
            Vbo::Buffering(_, vec) => vec.extend(other),
            _ => panic!("Vbo must not be uploaded yet!"),
        }
    }

    pub fn upload(&mut self, target: u32, usage: u32) {
        match self {
            Vbo::Buffering(gl, vec) => {
                let slice = &vec;
                let bytes: &[u8] = unsafe {
                    core::slice::from_raw_parts(
                        slice.as_ptr() as *const u8,
                        slice.len() * core::mem::size_of::<T>(),
                    )
                };
                let vbo = gl.create_buffer().unwrap();
                gl.bind_buffer(target, Some(&vbo));
                gl.buffer_data_with_u8_array(target, bytes, usage);
                *self = Vbo::Uploaded(vbo);
            }
            _ => panic!("Vbo must not be uploaded yet!"),
        }
    }

    pub fn update(&self, gl: &glow::Context, offset: i32, slice: &[T]) {
        let size = core::mem::size_of::<T>();
        let bytes: &[u8] =
            unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len() * size) };
        gl.buffer_sub_data_with_i32_and_u8_array(glow::ARRAY_BUFFER, offset * size as i32, bytes);
    }
}
