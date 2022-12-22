use crate::tga;
use serde::{Deserialize, Serialize};
use std::io;

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
#[serde(deny_unknown_fields, tag = "param_name")]
pub enum Binding {
    #[serde(rename = "zSort")]
    ZSort {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.t.x")]
    TransformTX {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.t.y")]
    TransformTY {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.s.x")]
    TransformSX {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.s.y")]
    TransformSY {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.r.x")]
    TransformRX {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.r.y")]
    TransformRY {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "transform.r.z")]
    TransformRZ {
        node: u32,
        values: Vec<Vec<f32>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
    #[serde(rename = "deform")]
    Deform {
        node: u32,
        values: Vec<Vec<Vec<[f32; 2]>>>,
        #[serde(rename = "isSet")]
        is_set: Vec<Vec<bool>>,
        interpolate_mode: InterpolateMode,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    uuid: u32,
    pub name: String,
    is_vec2: bool,
    min: [f32; 2],
    max: [f32; 2],
    defaults: [f32; 2],
    axis_points: [Vec<f32>; 2],
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
            writer.write_all(&data)?;
        }
        Ok(())
    }
}
