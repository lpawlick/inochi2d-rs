use nom::{
    bytes::complete::tag,
    multi::{length_data, length_value},
    number::complete::{be_u32, be_u8},
    IResult,
};
use serde::Deserialize;

const MAGIC: &[u8] = b"TRNSRTS\0";
const TEX: &[u8] = b"TEX_SECT";

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Physics {
    pixels_per_meter: f32,
    gravity: f32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transform {
    pub trans: [f32; 3],
    rot: [f32; 3],
    scale: [f32; 2],
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mesh {
    pub verts: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<u16>,
    origin: [f32; 2],
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
enum MaskMode {
    Mask,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Mask {
    pub source: u32,
    mode: MaskMode,
}

#[derive(Debug, Deserialize)]
pub enum ModelType {
    SpringPendulum,
}

#[derive(Debug, Deserialize)]
pub enum MapMode {
    XY,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum BlendMode {
    Multiply,
    Normal,
    ColorDodge,
    LinearDodge,
    Screen,
    ClipToLower,
}

#[derive(Debug, Deserialize)]
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
        opacity: Option<f32>,
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

#[derive(Debug, Deserialize)]
pub enum InterpolateMode {
    Linear,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
pub struct Texture {
    pub data: Vec<u8>,
}

impl Texture {
    fn parse(i: &[u8]) -> IResult<&[u8], Texture> {
        let (i, format) = be_u8(i)?;
        match format {
            1 => {
                let data = i.to_vec();
                Ok((b"", Texture { data }))
            }
            _ => todo!("Unknown format {format}!"),
        }
    }
}

fn be_u32_plus_1(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, int) = be_u32(i)?;
    Ok((i, 1 + int))
}

#[derive(Debug)]
pub struct Model {
    pub puppet: Puppet,
    pub textures: Vec<Texture>,
}

impl Model {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Model> {
        let (i, _) = tag(MAGIC)(i)?;
        let (i, json) = length_data(be_u32)(i)?;

        let (i, _) = tag(TEX)(i)?;
        let (mut i, num_textures) = be_u32(i)?;
        let mut textures = Vec::new();
        for _ in 0..num_textures {
            let (i2, texture) = length_value(be_u32_plus_1, Texture::parse)(i)?;
            textures.push(texture);
            i = i2;
        }

        let puppet = serde_json::from_slice(json).unwrap();
        Ok((i, Model { puppet, textures }))
    }
}
