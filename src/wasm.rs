// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::glow;
use crate::{gl, Model, TextureReceiver};
use js_sys::{Array, Boolean, JsString, Object};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct JsModel(Model);

#[wasm_bindgen]
pub fn parse(buf: &[u8]) -> JsModel {
    let model = Model::parse(buf).unwrap();
    JsModel(model)
}

#[wasm_bindgen]
pub struct JsTextureReceiver(TextureReceiver);

#[wasm_bindgen]
pub fn decode_textures(model: &mut JsModel) -> JsTextureReceiver {
    JsTextureReceiver(model.0.decode_textures())
}

#[wasm_bindgen]
pub struct JsContext {
    gl: glow::Context,
    width: u32,
    height: u32,
    has_astc: bool,
    has_bptc: bool,
}

#[wasm_bindgen]
pub fn has_astc(JsContext { has_astc, .. }: &JsContext) -> bool {
    *has_astc
}

#[wasm_bindgen]
pub fn has_bptc(JsContext { has_bptc, .. }: &JsContext) -> bool {
    *has_bptc
}

#[wasm_bindgen]
pub fn setup_context(id: &str) -> Result<JsContext, JsValue> {
    let window = web_sys::window().ok_or(JsValue::NULL)?;
    let scale = window.device_pixel_ratio();
    let document = window.document().ok_or(JsValue::NULL)?;
    let canvas = document
        .get_element_by_id(id)
        .ok_or(JsValue::from_str("Canvas not found!"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .or(Err(JsValue::from_str("Not a canvas element!")))?;
    let (width, height) = if scale != 1.0 {
        let width = canvas.width();
        let height = canvas.height();
        let scaled_width = (width as f64 * scale) as u32;
        let scaled_height = (height as f64 * scale) as u32;
        canvas.set_width(scaled_width);
        canvas.set_height(scaled_height);
        let style = canvas.style();
        style.set_property("width", &format!("{width}px"))?;
        style.set_property("height", &format!("{height}px"))?;
        (scaled_width, scaled_height)
    } else {
        (canvas.width(), canvas.height())
    };
    let params = Array::from_iter([
        JsValue::from(JsString::from("stencil")),
        JsValue::from(Boolean::from(true)),
    ]);
    let params = Array::from_iter([params]);
    let params = Object::from_entries(&params)?;
    let gl = canvas
        .get_context_with_context_options("webgl", &params)?
        .ok_or(JsValue::from_str("WebGL context creation failure"))?
        .dyn_into::<web_sys::WebGlRenderingContext>()?;

    let mut has_astc = false;
    let mut has_bptc = false;
    if let Some(exts) = gl.get_supported_extensions() {
        if exts.includes(&JsString::from("WEBGL_compressed_texture_astc"), 0) {
            if let Ok(Some(ext)) = gl.get_extension("WEBGL_compressed_texture_astc") {
                let ext = ext.unchecked_into::<web_sys::WebglCompressedTextureAstc>();
                if let Some(profiles) = ext.get_supported_profiles() {
                    if profiles.includes(&JsString::from("ldr"), 0) {
                        has_astc = true;
                    }
                }
            }
        }
        if exts.includes(&JsString::from("EXT_texture_compression_bptc"), 0) {
            if let Ok(Some(_)) = gl.get_extension("EXT_texture_compression_bptc") {
                has_bptc = true;
            }
        }
    }

    Ok(JsContext {
        gl,
        width,
        height,
        has_astc,
        has_bptc,
    })
}

#[wasm_bindgen]
pub fn setup(context: &JsContext, model: &JsModel, textures: JsTextureReceiver) {
    let JsContext {
        gl, width, height, ..
    } = context;
    let renderer = gl::setup(gl, &model.0.puppet.nodes, textures.0, *width, *height);
    renderer.clear();
    let num_nodes = gl::count_nodes(&model.0.puppet.nodes);
    let order = gl::sort_nodes_by_zsort(num_nodes, &model.0.puppet.nodes);
    renderer.render_nodes(&order);
}
