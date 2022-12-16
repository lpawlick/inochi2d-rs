use serde::{Deserialize, Serialize};
use std::io;

const MAGIC: &[u8] = b"TRNSRTS\0";
const TEX: &[u8] = b"TEX_SECT";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Meta {
    pub name: Option<String>,
    pub version: String,
    pub rigger: String,
    pub artist: String,
    pub rights: Option<String>,
    pub copyright: String,
    #[serde(rename = "licenseURL")]
    pub license_url: String,
    pub contact: String,
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
pub enum Texture {
    Png(Vec<u8>),
    Tga(Vec<u8>),
    Bc7(Vec<u8>),
    Decoded {
        width: u32,
        height: u32,
        data: Vec<u8>,
    },
}

impl Texture {
    pub fn decode(&mut self) {
        let (data, format) = match self {
            Texture::Png(data) => (data, image::ImageFormat::Png),
            Texture::Tga(data) => (data, image::ImageFormat::Tga),
            Texture::Bc7(data) => todo!("BC7 is still unimplemented"),
            // Nothing to do!
            Texture::Decoded { .. } => return,
        };
        let image = image::load_from_memory_with_format(data, format)
            .unwrap()
            .into_rgba8();
        let (width, height) = image.dimensions();
        *self = Texture::Decoded {
            width,
            height,
            data: image.to_vec(),
        };
    }

    pub fn encode(&mut self, format: image::ImageFormat) {
        match self {
            Texture::Decoded {
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
                *self = match format {
                    image::ImageFormat::Png => Texture::Png(buf.into_inner()),
                    image::ImageFormat::Tga => Texture::Tga(buf.into_inner()),
                    _ => panic!("Unsupported format {format:?}"),
                }
            }
            _ => panic!("Unsupported image: {self:?}"),
        }
    }
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
    pub textures: Vec<Texture>,
}

impl Model {
    pub fn parse<R: io::Read>(mut reader: R) -> io::Result<Model> {
        let magic = read_array::<R, 8>(&mut reader)?;
        if magic != MAGIC {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let length = read_be_u32(&mut reader)?;
        let json = read_vec(&mut reader, length)?;
        let puppet = serde_json::from_slice(&json).unwrap();

        let magic = read_array::<R, 8>(&mut reader)?;
        if magic != TEX {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let num_textures = read_be_u32(&mut reader)?;
        let mut textures = Vec::new();
        for _ in 0..num_textures {
            let length = read_be_u32(&mut reader)?;
            let format = read_array::<R, 1>(&mut reader)?[0];
            let data = read_vec(&mut reader, length)?;
            let texture = match format {
                0 => Texture::Png(data),
                1 => Texture::Tga(data),
                2 => Texture::Bc7(data),
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
                Texture::Png(data) => (0u8, data),
                Texture::Tga(data) => (1u8, data),
                Texture::Bc7(data) => (2u8, data),
                Texture::Decoded { .. } => todo!("Automated encoding of textures unsupported."),
            };
            writer.write_all(&(data.len() as u32).to_be_bytes())?;
            writer.write_all(&[format])?;
            writer.write_all(&data)?;
        }
        Ok(())
    }
}
