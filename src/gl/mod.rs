use crate::{BlendMode, CompressedTexture, Mask, Model, Node, Texture, Transform};
use glfw::{Action, Context, Key};
use glow::HasContext;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::mpsc;

mod vbo;
use vbo::Vbo;

mod program;
use program::Program;

const SIZE: u32 = 2048;

const VERTEX: &str = "#version 100
precision mediump float;
uniform vec2 trans;
attribute vec2 pos;
attribute vec2 uvs;
attribute vec2 deform;
varying vec2 texcoord;

void main() {
    vec2 pos2 = pos + trans + deform;
    pos2.y = -pos2.y;
    texcoord = vec2(uvs.x, -uvs.y);
    gl_Position = vec4(pos2 / 3072.0, 0.0, 1.0);
}
";

const FRAGMENT: &str = "#version 100
precision mediump float;
uniform sampler2D texture;
varying vec2 texcoord;

void main() {
    vec4 color = texture2D(texture, texcoord);
    if (color.a < 0.05) {
        discard;
    }
    gl_FragColor = color;
}
";

const VERTEX_PASSTHROUGH: &str = "#version 100
precision mediump float;
attribute vec2 pos;
attribute vec2 uvs;
varying vec2 texcoord;

void main() {
    texcoord = uvs;
    gl_Position = vec4(pos, 0.0, 1.0);
}
";

const FRAGMENT_PASSTHROUGH: &str = "#version 100
precision mediump float;
uniform sampler2D texture;
varying vec2 texcoord;

void main() {
    gl_FragColor = texture2D(texture, texcoord);
}
";

struct Locations {
    trans: Option<glow::NativeUniformLocation>,
}

impl Locations {
    fn new() -> Locations {
        Locations { trans: None }
    }
}

struct MutableStuff {
    prev_program: Option<glow::NativeProgram>,
    prev_stencil: bool,
    prev_blend_mode: Option<BlendMode>,
    prev_texture: Option<glow::NativeTexture>,
    prev_masks: Vec<Mask>,
}

struct GlRenderer<'a> {
    gl: &'a glow::Context,
    nodes: BTreeMap<u32, EnumNode>,
    mutable: RefCell<MutableStuff>,
    current_ibo_offset: u16,
    verts: Vbo<'a, f32>,
    uvs: Vbo<'a, f32>,
    deform: Vbo<'a, f32>,
    ibo: Vbo<'a, u16>,
    textures: Vec<glow::NativeTexture>,
    part_program: Program<'a>,
    locations: Locations,
    composite_program: Program<'a>,
    composite_fbo: glow::NativeFramebuffer,
    composite_texture: glow::NativeTexture,
}

impl<'a> GlRenderer<'a> {
    fn new(gl: &'a glow::Context) -> Result<GlRenderer, String> {
        let part_program = Program::builder(&gl)?
            .shader(glow::VERTEX_SHADER, VERTEX)?
            .shader(glow::FRAGMENT_SHADER, FRAGMENT)?
            .link()?;
        let mut locations = Locations::new();
        locations.trans = part_program.get_uniform_location("trans");

        let composite_program = Program::builder(&gl)?
            .shader(glow::VERTEX_SHADER, VERTEX_PASSTHROUGH)?
            .shader(glow::FRAGMENT_SHADER, FRAGMENT_PASSTHROUGH)?
            .link()?;

        let verts = Vbo::from(
            gl,
            vec![-1., -1., -1., 1., 1., -1., 1., -1., -1., 1., 1., 1.],
        );
        let uvs = Vbo::from(gl, vec![0., 0., 0., 1., 1., 0., 1., 0., 0., 1., 1., 1.]);
        let deform = Vbo::from(gl, vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        let ibo = Vbo::new(gl);

        let composite_texture;
        let composite_fbo;
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 0.0);
            gl.enable(glow::BLEND);
            gl.stencil_mask(0xff);

            composite_texture = Self::upload_texture(&gl, SIZE, SIZE, None);
            composite_fbo = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(composite_fbo));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(composite_texture),
                0,
            );
            assert_eq!(
                gl.check_framebuffer_status(glow::FRAMEBUFFER),
                glow::FRAMEBUFFER_COMPLETE
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        };

        let mutable = RefCell::new(MutableStuff {
            prev_program: None,
            prev_stencil: false,
            prev_blend_mode: None,
            prev_texture: None,
            prev_masks: Vec::new(),
        });

        let nodes = BTreeMap::new();
        Ok(GlRenderer {
            gl,
            nodes,
            mutable,
            current_ibo_offset: 6,
            verts,
            uvs,
            deform,
            ibo,
            locations,
            textures: Vec::new(),
            part_program,
            composite_program,
            composite_texture,
            composite_fbo,
        })
    }

    fn flatten_nodes(&mut self, node: &Node, parent: Option<u32>) {
        match *node {
            Node::Node {
                uuid,
                ref transform,
                ref children,
                ..
            } => {
                let transform = transform.clone();
                let simple_node = SimpleNode { parent, transform };
                self.push(uuid, EnumNode::Node(simple_node));
                for child in children.iter() {
                    self.flatten_nodes(child, Some(uuid));
                }
            }
            Node::SimplePhysics { uuid, .. } => {
                self.push(uuid, EnumNode::SimplePhysics);
            }
            Node::Part {
                uuid,
                ref mesh,
                ref transform,
                blend_mode,
                textures,
                ref masks,
                ref children,
                ref name,
                ..
            } => {
                let num_verts = mesh.verts.len();
                assert_eq!(num_verts, mesh.uvs.len());

                let start_indice = self.ibo.len() as u16;
                let num_indices = mesh.indices.len() as u16;
                let start_deform = self.current_ibo_offset * 2;
                self.verts.extend_from_slice(mesh.verts.as_slice());
                self.uvs.extend_from_slice(mesh.uvs.as_slice());
                self.deform
                    .extend_from_slice(vec![0.; num_verts].as_slice());
                self.ibo.extend(
                    mesh.indices
                        .iter()
                        .map(|index| index + self.current_ibo_offset),
                );
                self.current_ibo_offset += (num_verts / 2) as u16;

                let parent = parent.unwrap();
                let transform = transform.clone();
                let masks = masks.clone();
                let name = name.clone();

                let part = Part {
                    start_indice,
                    num_indices,
                    start_deform,
                    transform,
                    blend_mode,
                    textures,
                    parent,
                    masks,
                    name,
                };
                self.push(uuid, EnumNode::Part(part));
                for child in children.iter() {
                    self.flatten_nodes(child, Some(uuid));
                }
            }
            Node::Composite {
                uuid,
                ref transform,
                blend_mode,
                ref children,
                ref name,
                ..
            } => {
                let parent = parent.unwrap();
                let transform = transform.clone();
                let children_uuid = children.iter().flat_map(collect_children_uuids).collect();
                let name = name.clone();

                let composite = Composite {
                    transform,
                    blend_mode,
                    parent,
                    children: children_uuid,
                    name,
                };
                self.push(uuid, EnumNode::Composite(composite));
                for child in children.iter() {
                    self.flatten_nodes(child, Some(uuid));
                }
            }
        }
    }

    unsafe fn upload_texture(
        gl: &glow::Context,
        width: u32,
        height: u32,
        data: Option<&[u8]>,
    ) -> glow::NativeTexture {
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::BGRA as i32,
            width as i32,
            height as i32,
            0,
            glow::BGRA,
            glow::UNSIGNED_BYTE,
            data,
        );
        texture
    }

    fn load_texture(&self, tex: &Texture) -> glow::NativeTexture {
        match tex {
            Texture::Rgba {
                width,
                height,
                data,
            } => unsafe { Self::upload_texture(self.gl, *width, *height, Some(data)) },
        }
    }

    fn upload_textures(&mut self, rx: mpsc::Receiver<(usize, Texture)>, num_textures: usize) {
        let mut vec = vec![None; num_textures];
        while let Ok((i, tex)) = rx.recv() {
            let texture = self.load_texture(&tex);
            vec[i] = Some(texture);
        }
        self.textures = vec.into_iter().map(Option::unwrap).collect();
    }

    fn upload_buffers(&mut self) {
        let gl = &self.gl;

        unsafe {
            self.verts.upload(glow::ARRAY_BUFFER, glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);
            gl.enable_vertex_attrib_array(0);

            self.uvs.upload(glow::ARRAY_BUFFER, glow::STATIC_DRAW);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 8, 0);
            gl.enable_vertex_attrib_array(1);

            self.deform.upload(glow::ARRAY_BUFFER, glow::DYNAMIC_DRAW);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8, 0);
            gl.enable_vertex_attrib_array(2);

            self.ibo
                .upload(glow::ELEMENT_ARRAY_BUFFER, glow::STATIC_DRAW);
        }
    }

    fn push(&mut self, uuid: u32, buf: EnumNode) {
        self.nodes.insert(uuid, buf);
    }

    fn get(&self, uuid: u32) -> Option<&EnumNode> {
        self.nodes.get(&uuid)
    }

    unsafe fn set_stencil(&self, stencil: bool) {
        let prev = &mut self.mutable.borrow_mut().prev_stencil;
        if *prev == stencil {
            return;
        }
        let gl = &self.gl;
        if stencil {
            gl.enable(glow::STENCIL_TEST);
        } else {
            gl.disable(glow::STENCIL_TEST);
        }
        *prev = stencil;
    }

    unsafe fn use_program(&self, program: &Program) {
        let prev = &mut self.mutable.borrow_mut().prev_program;
        if *prev == Some(program.program) {
            return;
        }
        program.use_();
        *prev = Some(program.program);
    }

    unsafe fn bind_texture(&self, texture: glow::NativeTexture) {
        let prev = &mut self.mutable.borrow_mut().prev_texture;
        if *prev == Some(texture) {
            return;
        }
        let gl = &self.gl;
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        *prev = Some(texture);
    }

    unsafe fn set_blend_mode(&self, mode: BlendMode) {
        let prev = &mut self.mutable.borrow_mut().prev_blend_mode;
        if *prev == Some(mode) {
            return;
        }
        let gl = &self.gl;
        match mode {
            BlendMode::Normal => gl.blend_func(glow::ONE, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::Multiply => gl.blend_func(glow::DST_COLOR, glow::ONE_MINUS_SRC_ALPHA),
            BlendMode::ColorDodge => gl.blend_func(glow::DST_COLOR, glow::ONE),
            BlendMode::LinearDodge => gl.blend_func(glow::ONE, glow::ONE),
            BlendMode::Screen => gl.blend_func(glow::ONE, glow::ONE_MINUS_SRC_COLOR),
            BlendMode::ClipToLower => gl.blend_func(glow::DST_ALPHA, glow::ONE_MINUS_SRC_ALPHA),
        }
        *prev = Some(mode);
    }

    unsafe fn recompute_masks(&self, part: &Part) {
        if self.mutable.borrow().prev_masks == part.masks {
            return;
        }

        let gl = &self.gl;
        self.set_stencil(true);
        gl.color_mask(false, false, false, false);
        gl.stencil_op(glow::KEEP, glow::KEEP, glow::REPLACE);
        gl.stencil_func(glow::ALWAYS, 0xff, 0xff);
        gl.clear(glow::STENCIL_BUFFER_BIT);
        for mask in part.masks.iter() {
            match self.get(mask.source).unwrap() {
                EnumNode::Part(part) => self.render_part(part),
                _ => panic!("Only parts allowed in masks, for now."),
            }
        }
        gl.color_mask(true, true, true, true);
        gl.stencil_func(glow::EQUAL, 0xff, 0xff);
        gl.stencil_op(glow::KEEP, glow::KEEP, glow::KEEP);

        self.mutable.borrow_mut().prev_masks = part.masks.clone();
    }

    unsafe fn render_part(&self, part: &Part) {
        self.use_program(&self.part_program);

        if !part.masks.is_empty() {
            self.recompute_masks(part);
        }

        let trans = part.trans(self);

        let gl = &self.gl;
        self.bind_texture(self.textures[part.textures[0]]);
        self.set_blend_mode(part.blend_mode);
        gl.uniform_2_f32(self.locations.trans.as_ref(), trans[0], trans[1]);

        gl.draw_elements(
            glow::TRIANGLES,
            part.num_indices as i32,
            glow::UNSIGNED_SHORT,
            (part.start_indice as i32) * 2,
        );
    }

    unsafe fn render_composite(&self, composite: &Composite) {
        let gl = &self.gl;
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.composite_fbo));
        gl.clear(glow::COLOR_BUFFER_BIT);
        self.render_nodes(&composite.children);

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        self.bind_texture(self.composite_texture);
        self.set_blend_mode(composite.blend_mode);
        self.use_program(&self.composite_program);
        gl.draw_arrays(glow::TRIANGLES, 0, 6);
    }

    fn render_nodes(&self, order: &[u32]) {
        let gl = self.gl;
        for &uuid in order {
            match self.get(uuid).unwrap() {
                EnumNode::Part(part) => unsafe {
                    gl.push_debug_group(glow::DEBUG_SOURCE_APPLICATION, 0, &part.name);
                    self.set_stencil(false);
                    self.render_part(part);
                    gl.pop_debug_group();
                },
                EnumNode::Composite(composite) => unsafe {
                    gl.push_debug_group(glow::DEBUG_SOURCE_APPLICATION, 0, &composite.name);
                    self.render_composite(composite);
                    gl.pop_debug_group();
                },
                EnumNode::Node(_) => (),
                EnumNode::SimplePhysics => (),
            }
        }
    }

    fn clear(&self) {
        let gl = &self.gl;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}

struct Composite {
    transform: Transform,
    blend_mode: BlendMode,
    parent: u32,
    children: Vec<u32>,
    name: String,
}

struct SimpleNode {
    transform: Transform,
    parent: Option<u32>,
}

enum EnumNode {
    Part(Part),
    Composite(Composite),
    Node(SimpleNode),
    SimplePhysics,
}

#[derive(Debug)]
struct Part {
    start_indice: u16,
    num_indices: u16,
    start_deform: u16,
    transform: Transform,
    textures: [usize; 3],
    blend_mode: BlendMode,
    parent: u32,
    masks: Vec<Mask>,
    name: String,
}

impl Part {
    fn trans(&self, nodes: &GlRenderer) -> [f32; 3] {
        let mut trans = self.transform.trans;
        let mut parent_uuid = self.parent;
        while let Some(parent_node) = nodes.get(parent_uuid) {
            let (parent, parent_trans) = match parent_node {
                EnumNode::Part(node) => (node.parent, node.transform.trans),
                EnumNode::Composite(node) => (node.parent, node.transform.trans),
                EnumNode::Node(node) => (
                    match node.parent {
                        Some(parent) => parent,
                        None => break,
                    },
                    node.transform.trans,
                ),
                _ => break,
            };
            trans[0] += parent_trans[0];
            trans[1] += parent_trans[1];
            trans[2] += parent_trans[2];
            parent_uuid = parent;
        }
        trans
    }
}

fn collect_children_uuids(node: &Node) -> Vec<u32> {
    let mut uuids = vec![node.uuid()];
    match node {
        Node::Node { children, .. }
        | Node::Part { children, .. }
        | Node::Composite { children, .. } => {
            for child in children.iter() {
                uuids.extend(collect_children_uuids(child));
            }
        }
        _ => (),
    }
    uuids
}

fn recurse(node: &Node, zsort: f32) -> Vec<(u32, f32)> {
    let zsort = zsort + node.zsort();
    let mut vec = vec![(node.uuid(), zsort)];
    if let Node::Node { children, .. } | Node::Part { children, .. } = node {
        for child in children.iter() {
            vec.extend(recurse(child, zsort));
        }
    }
    vec
}

fn sort_uuids_by_zsort(mut uuids: Vec<(u32, f32)>) -> Vec<u32> {
    uuids.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());
    uuids.into_iter().map(|(uuid, zsort)| uuid).collect()
}

fn sort_nodes_by_zsort(node: &Node) -> Vec<u32> {
    let uuids = recurse(node, 0.);
    sort_uuids_by_zsort(uuids)
}

#[cfg(feature = "parallel")]
fn decode_textures(textures: &mut Vec<CompressedTexture>) -> mpsc::Receiver<(usize, Texture)> {
    let mut num_threads = std::thread::available_parallelism().unwrap().get();
    if num_threads > 1 {
        num_threads -= 1;
    }
    if num_threads > textures.len() {
        num_threads = textures.len();
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

    for ((i, tex), tx) in textures.drain(..).enumerate().zip(pipes.iter().cycle()) {
        tx.send((i, tex)).unwrap();
    }

    rx2
}

#[cfg(not(feature = "parallel"))]
fn decode_textures(textures: &mut Vec<CompressedTexture>) -> mpsc::Receiver<(usize, Texture)> {
    let (tx, rx) = mpsc::channel();
    for (i, tex) in textures.drain(..).enumerate() {
        let tex = tex.decode();
        tx.send((i, tex)).unwrap();
    }
    rx
}

pub fn render(model: &mut Model) {
    // We start decoding textures on threads…
    let num_textures = model.textures.len();
    let rx = decode_textures(&mut model.textures);

    let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(2, 0));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));
    glfw.window_hint(glfw::WindowHint::Decorated(false));

    let (mut window, events) = glfw
        .create_window(SIZE, SIZE, "inochi2d", glfw::WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_key_polling(true);
    let gl =
        unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };

    let mut renderer = GlRenderer::new(&gl).unwrap();
    renderer.flatten_nodes(&model.puppet.nodes, None);
    renderer.upload_buffers();

    // … So that here hopefully some have already been decoded, while we were setting up GLES.
    renderer.upload_textures(rx, num_textures);

    let order = sort_nodes_by_zsort(&model.puppet.nodes);
    while !window.should_close() {
        renderer.clear();
        renderer.render_nodes(&order);
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
