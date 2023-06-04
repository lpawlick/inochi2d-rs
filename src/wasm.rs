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
use crate::ParamValues;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct JsModel {
    model: Model,
}

#[wasm_bindgen]
pub fn parse(buf: &[u8]) -> JsModel {
    let model = Model::parse(buf).unwrap();
    JsModel { model }
}

#[wasm_bindgen]
pub struct JsTextureReceiver {
    receiver: TextureReceiver,
}

#[wasm_bindgen]
pub fn decode_textures(model: &mut JsModel) -> JsTextureReceiver 
{
    JsTextureReceiver { receiver: model.model.decode_textures() }
}

#[wasm_bindgen]
pub struct JsGlRenderer 
{
    model: JsModel,
    renderer: gl::GlRenderer<'static>,
}

#[wasm_bindgen]
impl JsGlRenderer 
{
    pub fn animate(&mut self, params: js_sys::Object) 
    {
        let mut param_map: HashMap<String, [f32; 2]> = HashMap::new();
        let keys = js_sys::Reflect::own_keys(&params).expect("Failed to get object keys");
        for key in keys.iter() {
            if let Some(key_str) = key.as_string() {
                if let Ok(js_value) = js_sys::Reflect::get(&params, &key) {
                    if let Ok(js_arr) = js_value.dyn_into::<js_sys::Array>() {
                        let x = js_arr.get(0).as_f64().unwrap_or(0.0) as f32;
                        let y = js_arr.get(1).as_f64().unwrap_or(0.0) as f32;
                        param_map.insert(key_str, [x, y]);
                    }
                }
            }
        }

        let mut param_values = ParamValues::new(&self.model.model.puppet.param);

        // Iterate over all parameters and update them if found
        for (key, value) in param_map.iter() 
        {
            param_values.set(key, *value);
        }
        self.renderer.animate(&param_values);
    }

    pub fn clear(&self) 
    {
        self.renderer.clear();
    }

    pub fn render(&self) 
    {
        let num_nodes = gl::count_nodes(&self.model.model.puppet.nodes);
        let order = gl::sort_nodes_by_zsort(num_nodes, &self.model.model.puppet.nodes);
        self.renderer.render_nodes(&order);
    }
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
    let document = window.document().ok_or(JsValue::NULL)?;
    let canvas = document
        .get_element_by_id(id)
        .ok_or(JsValue::from_str("Canvas not found!"))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .or(Err(JsValue::from_str("Not a canvas element!")))?;
    let width = canvas.width();
    let height = canvas.height();
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
pub fn setup(context: JsContext, model: JsModel, textures: JsTextureReceiver) -> Result<JsGlRenderer, JsValue> {
    let JsContext {
        gl, width, height, ..
    } = context;
    // This creates a memory leak!
    let gl_static = Box::leak(Box::new(gl));
    let renderer = gl::setup(gl_static, &model.model.puppet.nodes, textures.receiver, width, height);
    renderer.clear();
    let num_nodes = gl::count_nodes(&model.model.puppet.nodes);
    let order = gl::sort_nodes_by_zsort(num_nodes, &model.model.puppet.nodes);
    renderer.render_nodes(&order);
    Ok(JsGlRenderer { renderer, model })
}