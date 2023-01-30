// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::tga;
use serde::{Deserialize, Serialize};
use std::io;
use std::sync::mpsc;

const MAGIC: &[u8] = b"TRNSRTS\0";
const TEX: &[u8] = b"TEX_SECT";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    pub name: Option<String>,
    pub version: String,
    pub rigger: Option<String>,
    pub artist: Option<String>,
    pub rights: Option<String>,
    pub copyright: Option<String>,
    #[serde(rename = "licenseURL")]
    pub license_url: Option<String>,
    pub contact: Option<String>,
    pub reference: Option<String>,
    thumbnail_id: u32,
    preserve_pixels: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Physics {
    pixels_per_meter: f32,
    gravity: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Transform {
    pub trans: [f32; 3],
    rot: [f32; 3],
    scale: [f32; 2],
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    pub verts: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<u16>,
    origin: [f32; 2],
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
enum MaskMode {
    Mask,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mask {
    pub source: u32,
    mode: MaskMode,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ModelType {
    SpringPendulum,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MapMode {
    XY,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Multiply,
    Normal,
    ColorDodge,
    LinearDodge,
    Screen,
    ClipToLower,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, tag = "type")]
pub enum Node {
    Node {
        uuid: u32,
        name: String,
        enabled: bool,
        zsort: f32,
        transform: Transform,
        #[serde(rename = "lockToRoot")]
        lock_to_root: bool,
        children: Vec<Node>,
    },
    Part {
        uuid: u32,
        name: String,
        enabled: bool,
        zsort: f32,
        transform: Transform,
        #[serde(rename = "lockToRoot")]
        lock_to_root: bool,
        #[serde(default)]
        children: Vec<Node>,
        textures: [usize; 3],
        mesh: Mesh,
        blend_mode: BlendMode,
        opacity: f32,
        tint: [f32; 3],
        #[serde(rename = "screenTint")]
        screen_tint: [f32; 3],
        mask_threshold: f32,
        #[serde(default)]
        masks: Vec<Mask>,
        #[serde(rename = "psdLayerPath")]
        psd_layer_path: String,
    },
    Composite {
        uuid: u32,
        name: String,
        enabled: bool,
        zsort: f32,
        transform: Transform,
        #[serde(rename = "lockToRoot")]
        lock_to_root: bool,
        children: Vec<Node>,
        blend_mode: BlendMode,
        opacity: f32,
        tint: [f32; 3],
        #[serde(rename = "screenTint")]
        screen_tint: [f32; 3],
        mask_threshold: f32,
    },
    SimplePhysics {
        uuid: u32,
        name: String,
        enabled: bool,
        zsort: f32,
        transform: Transform,
        #[serde(rename = "lockToRoot")]
        lock_to_root: bool,
        param: u32,
        model_type: ModelType,
        map_mode: MapMode,
        gravity: f32,
        length: f32,
        frequency: f32,
        angle_damping: f32,
        length_damping: f32,
        output_scale: [f32; 2],
    },
}

impl Node {
    pub fn uuid(&self) -> u32 {
        match *self {
            Node::Node { uuid, .. }
            | Node::Part { uuid, .. }
            | Node::Composite { uuid, .. }
            | Node::SimplePhysics { uuid, .. } => uuid,
        }
    }

    pub fn enabled(&self) -> bool {
        match *self {
            Node::Node { enabled, .. }
            | Node::Part { enabled, .. }
            | Node::Composite { enabled, .. }
            | Node::SimplePhysics { enabled, .. } => enabled,
        }
    }

    pub fn zsort(&self) -> f32 {
        match *self {
            Node::Node { zsort, .. }
            | Node::Part { zsort, .. }
            | Node::Composite { zsort, .. }
            | Node::SimplePhysics { zsort, .. } => zsort,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum InterpolateMode {
    Nearest,
    Linear,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, tag = "param_name", content = "values")]
pub enum BindingValues {
    #[serde(rename = "zSort")]
    ZSort(Vec<Vec<f32>>),
    #[serde(rename = "transform.t.x")]
    TransformTX(Vec<Vec<f32>>),
    #[serde(rename = "transform.t.y")]
    TransformTY(Vec<Vec<f32>>),
    #[serde(rename = "transform.t.z")]
    TransformTZ(Vec<Vec<f32>>),
    #[serde(rename = "transform.s.x")]
    TransformSX(Vec<Vec<f32>>),
    #[serde(rename = "transform.s.y")]
    TransformSY(Vec<Vec<f32>>),
    #[serde(rename = "transform.r.x")]
    TransformRX(Vec<Vec<f32>>),
    #[serde(rename = "transform.r.y")]
    TransformRY(Vec<Vec<f32>>),
    #[serde(rename = "transform.r.z")]
    TransformRZ(Vec<Vec<f32>>),
    #[serde(rename = "deform")]
    Deform(Vec<Vec<Vec<[f32; 2]>>>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub node: u32,
    #[serde(flatten)]
    values: BindingValues,
    #[serde(rename = "isSet")]
    is_set: Vec<Vec<bool>>,
    interpolate_mode: InterpolateMode,
}

#[derive(Debug)]
pub enum Anim {
    ZSort(f32),
    TransformTX(f32),
    TransformTY(f32),
    TransformTZ(f32),
    TransformSX(f32),
    TransformSY(f32),
    TransformRX(f32),
    TransformRY(f32),
    TransformRZ(f32),
    Deform(Vec<f32>),
}

impl Binding {
    pub fn interpolate(&self, axis_points: &[Vec<f32>; 2], pos: [f32; 2]) -> Anim {
        assert!(pos[0] >= 0.);
        assert!(pos[1] >= 0.);
        assert!(pos[0] <= 1.);
        assert!(pos[1] <= 1.);

        match &self.values {
            BindingValues::Deform(values) => {
                let deform = interpolate_deform(values, axis_points, pos);
                let len = deform.len();
                let deform = unsafe {
                    let mut deform: Vec<f32> = core::mem::transmute(deform);
                    deform.set_len(len * 2);
                    deform
                };
                Anim::Deform(deform)
            }
            BindingValues::TransformTX(values) => {
                Anim::TransformTX(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::TransformTY(values) => {
                Anim::TransformTY(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::TransformSX(values) => {
                Anim::TransformSX(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::TransformSY(values) => {
                Anim::TransformSY(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::TransformRY(values) => {
                Anim::TransformRY(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::TransformRZ(values) => {
                Anim::TransformRZ(interpolate_f32(values, axis_points, pos))
            }
            BindingValues::ZSort(values) => Anim::ZSort(interpolate_f32(values, axis_points, pos)),
            val => panic!("{val:?}"),
        }
    }
}

fn get_step(pos: f32, index: usize, axis_points: &[f32]) -> f32 {
    let a = axis_points[index - 1];
    let b = axis_points[index];
    (pos - a) / (b - a)
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn mix_deform(a: &[[f32; 2]], b: &[[f32; 2]], t: f32) -> Vec<[f32; 2]> {
    a.iter()
        .zip(b.iter())
        .map(|([ax, ay], [bx, by])| [mix(*ax, *bx, t), mix(*ay, *by, t)])
        .collect()
}

// TODO: merge the next two functions into one, they are almost the same just operating on
// different types.

fn interpolate_deform(
    values: &[Vec<Vec<[f32; 2]>>],
    axis_points: &[Vec<f32>; 2],
    pos: [f32; 2],
) -> Vec<[f32; 2]> {
    for (i, x) in axis_points[0].iter().enumerate() {
        if *x == pos[0] {
            for (j, y) in axis_points[1].iter().enumerate() {
                if *y == pos[1] {
                    // No interpolation needed.
                    return values[i][j].clone();
                }
                if *y > pos[1] {
                    let a = &values[i][j - 1];
                    let b = &values[i][j];
                    let t = get_step(pos[1], j, &axis_points[1]);
                    return mix_deform(a, b, t);
                }
            }
            unreachable!();
        }
        if *x > pos[0] {
            for (j, y) in axis_points[1].iter().enumerate() {
                if *y == pos[1] {
                    let a = &values[i - 1][j];
                    let b = &values[i][j];
                    let t = get_step(pos[0], i, &axis_points[0]);
                    return mix_deform(a, b, t);
                }
                if *y > pos[1] {
                    let a = &values[i - 1][j - 1];
                    let b = &values[i][j - 1];
                    let t = get_step(pos[0], i, &axis_points[0]);
                    let y1 = mix_deform(a, b, t);

                    let a = &values[i - 1][j];
                    let b = &values[i][j];
                    let y2 = mix_deform(a, b, t);

                    let t = get_step(pos[1], j, &axis_points[1]);
                    return mix_deform(&y1, &y2, t);
                }
            }
            unreachable!();
        }
    }
    unreachable!();
}

fn interpolate_f32(values: &[Vec<f32>], axis_points: &[Vec<f32>; 2], pos: [f32; 2]) -> f32 {
    for (i, x) in axis_points[0].iter().enumerate() {
        if *x == pos[0] {
            for (j, y) in axis_points[1].iter().enumerate() {
                if *y == pos[1] {
                    // No interpolation needed.
                    return values[i][j];
                }
                if *y > pos[1] {
                    let a = values[i][j - 1];
                    let b = values[i][j];
                    let t = get_step(pos[1], j, &axis_points[1]);
                    return mix(a, b, t);
                }
            }
            unreachable!();
        }
        if *x > pos[0] {
            for (j, y) in axis_points[1].iter().enumerate() {
                if *y == pos[1] {
                    let a = values[i - 1][j];
                    let b = values[i][j];
                    let t = get_step(pos[0], i, &axis_points[0]);
                    return mix(a, b, t);
                }
                if *y > pos[1] {
                    let a = values[i - 1][j - 1];
                    let b = values[i][j - 1];
                    let t = get_step(pos[0], i, &axis_points[0]);
                    let y1 = mix(a, b, t);

                    let a = values[i - 1][j];
                    let b = values[i][j];
                    let y2 = mix(a, b, t);

                    let t = get_step(pos[1], j, &axis_points[1]);
                    return mix(y1, y2, t);
                }
            }
            unreachable!();
        }
    }
    unreachable!();
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    uuid: u32,
    pub name: String,
    is_vec2: bool,
    min: [f32; 2],
    max: [f32; 2],
    pub defaults: [f32; 2],
    pub axis_points: [Vec<f32>; 2],
    pub bindings: Vec<Binding>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Puppet {
    pub meta: Meta,
    physics: Physics,
    pub nodes: Node,
    automation: Option<serde_json::Value>,
    pub param: Vec<Param>,
    animations: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum CompressedTexture {
    Png(Vec<u8>),
    Tga(Vec<u8>),
    Bc7(Vec<u8>),
}

#[derive(Debug)]
pub enum Texture {
    Rgba {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}

impl CompressedTexture {
    pub fn decode(&self) -> Texture {
        match self {
            #[cfg(not(feature = "png"))]
            CompressedTexture::Png(_) => {
                panic!("PNG textures are unsupported without the png feature")
            }
            #[cfg(feature = "png")]
            CompressedTexture::Png(data) => {
                use image::ImageDecoder;
                let cursor = io::Cursor::new(data);
                let decoder = image::codecs::png::PngDecoder::new(cursor).unwrap();
                let (width, height) = decoder.dimensions();
                let mut data = vec![0u8; decoder.total_bytes() as usize];
                let color_type = decoder.color_type();
                decoder.read_image(&mut data).unwrap();
                let data = match color_type {
                    image::ColorType::Rgba8 => data,
                    image::ColorType::Rgb8 => {
                        let rgb = image::ImageBuffer::from_raw(width, height, data).unwrap();
                        let dynamic = image::DynamicImage::ImageRgb8(rgb);
                        let rgba = dynamic.into_rgba8();
                        rgba.into_vec()
                    }
                    _ => panic!("Unknown color type {color_type:?}"),
                };
                Texture::Rgba {
                    width,
                    height,
                    data,
                }
            }
            CompressedTexture::Tga(data) => {
                let (width, height, data) = tga::decode(data);
                Texture::Rgba {
                    width,
                    height,
                    data,
                }
            }
            CompressedTexture::Bc7(_) => todo!("BC7 is still unimplemented"),
        }
    }
}

impl Texture {
    #[cfg(feature = "encoding")]
    pub fn encode(&self, format: image::ImageFormat) -> CompressedTexture {
        match self {
            Texture::Rgba {
                width,
                height,
                data,
            } => {
                let buf = Vec::new();
                let mut buf = std::io::Cursor::new(buf);
                image::write_buffer_with_format(
                    &mut buf,
                    &data,
                    *width,
                    *height,
                    image::ColorType::Rgba8,
                    format,
                )
                .unwrap();
                match format {
                    image::ImageFormat::Png => CompressedTexture::Png(buf.into_inner()),
                    image::ImageFormat::Tga => CompressedTexture::Tga(buf.into_inner()),
                    _ => panic!("Unsupported format {format:?}"),
                }
            }
        }
    }
}

fn read_u8<R: io::Read>(reader: &mut R) -> io::Result<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_be_u32<R: io::Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_array<R: io::Read, const LENGTH: usize>(reader: &mut R) -> io::Result<[u8; LENGTH]> {
    let mut data = [0u8; LENGTH];
    reader.read_exact(&mut data)?;
    Ok(data)
}

fn read_vec<R: io::Read>(reader: &mut R, length: u32) -> io::Result<Vec<u8>> {
    let length = length as usize;
    let mut data = Vec::with_capacity(length);
    unsafe { data.set_len(length) };
    reader.read_exact(&mut data)?;
    Ok(data)
}

pub type TextureReceiver = (usize, mpsc::Receiver<(usize, Texture)>);

#[derive(Debug)]
pub struct Model {
    pub puppet: Puppet,
    pub textures: Vec<CompressedTexture>,
}

impl Model {
    pub fn parse<R: io::Read>(mut reader: R) -> io::Result<Model> {
        let magic = read_array::<R, 8>(&mut reader)?;
        if magic != MAGIC {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let puppet = {
            let length = read_be_u32(&mut reader)?;
            let json = read_vec(&mut reader, length)?;
            serde_json::from_slice(&json).unwrap()
        };

        let magic = read_array::<R, 8>(&mut reader)?;
        if magic != TEX {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let num_textures = read_be_u32(&mut reader)?;
        let mut textures = Vec::with_capacity(num_textures as usize);
        for _ in 0..num_textures {
            let length = read_be_u32(&mut reader)?;
            let format = read_u8(&mut reader)?;
            let data = read_vec(&mut reader, length)?;
            let texture = match format {
                0 => CompressedTexture::Png(data),
                1 => CompressedTexture::Tga(data),
                2 => CompressedTexture::Bc7(data),
                _ => panic!("Unknown format {format}"),
            };
            textures.push(texture);
        }

        Ok(Model { puppet, textures })
    }

    pub fn serialize<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(MAGIC)?;
        let json = serde_json::to_vec(&self.puppet)?;
        writer.write_all(&(json.len() as u32).to_be_bytes())?;
        writer.write_all(&json)?;
        writer.write_all(TEX)?;
        writer.write_all(&(self.textures.len() as u32).to_be_bytes())?;
        for texture in self.textures.iter() {
            let (format, data) = match texture {
                CompressedTexture::Png(data) => (0u8, data),
                CompressedTexture::Tga(data) => (1u8, data),
                CompressedTexture::Bc7(data) => (2u8, data),
            };
            writer.write_all(&(data.len() as u32).to_be_bytes())?;
            writer.write_all(&[format])?;
            writer.write_all(data)?;
        }
        Ok(())
    }

    #[cfg(feature = "parallel")]
    pub fn decode_textures(&mut self) -> TextureReceiver {
        let num = self.textures.len();
        let mut num_threads = std::thread::available_parallelism().unwrap().get();
        if num_threads > 1 {
            num_threads -= 1;
        }
        if num_threads > self.textures.len() {
            num_threads = self.textures.len();
        }

        let (tx2, rx2) = mpsc::channel();
        let mut pipes = Vec::with_capacity(num_threads);
        for _ in 0..num_threads {
            let (tx, rx) = mpsc::channel::<(usize, CompressedTexture)>();
            let tx2 = tx2.clone();
            std::thread::Builder::new()
                .name(String::from("Texture Decoder"))
                .spawn(move || {
                    while let Ok((i, tex)) = rx.recv() {
                        let tex = tex.decode();
                        tx2.send((i, tex)).unwrap();
                    }
                })
                .unwrap();
            pipes.push(tx);
        }

        for ((i, tex), tx) in self
            .textures
            .drain(..)
            .enumerate()
            .zip(pipes.iter().cycle())
        {
            tx.send((i, tex)).unwrap();
        }

        (num, rx2)
    }

    #[cfg(not(feature = "parallel"))]
    pub fn decode_textures(&mut self) -> TextureReceiver {
        let num = self.textures.len();
        let (tx, rx) = mpsc::channel();
        for (i, tex) in self.textures.drain(..).enumerate() {
            let tex = tex.decode();
            tx.send((i, tex)).unwrap();
        }
        (num, rx)
    }
}
