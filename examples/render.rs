// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glfw::{Action, Context, Key};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

fn print_info(meta: &inochi2d::Meta) {
    if let Some(ref name) = meta.name {
        println!("Model {name}");
    }
    println!("Version {}", meta.version);
    if let Some(ref rigger) = meta.rigger {
        println!("Rigger {rigger}");
    }
    if let Some(ref artist) = meta.artist {
        println!("Artist {artist}");
    }
    if let Some(ref rights) = meta.rights {
        println!("Rights {rights}");
    }
    if let Some(ref copyright) = meta.copyright {
        println!("Copyright {copyright}");
    }
    if let Some(ref license_url) = meta.license_url {
        println!("License {license_url}");
    }
    if let Some(ref contact) = meta.contact {
        println!("Contact {contact}");
    }
    if let Some(ref reference) = meta.reference {
        println!("Reference {reference}");
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <model.inp> [<width>×<height>]", args[0]);
        return;
    }
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
    let mut model = inochi2d::Model::parse(file).unwrap();
    print_info(&model.puppet.meta);
    let textures = model.decode_textures();

    let size = if args.len() == 3 {
        args[2].split_once('x').map(|(width, height)| {
            (
                u32::from_str(width).unwrap(),
                u32::from_str(height).unwrap(),
            )
        })
    } else {
        None
    };

    let (width, height) = size.unwrap_or((2048, 2048));

    let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
    glfw.window_hint(glfw::WindowHint::ContextVersion(2, 0));
    glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

    let (mut window, events) = glfw
        .create_window(width, height, "inochi2d", glfw::WindowMode::Windowed)
        .unwrap();
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let gl = inochi2d::glow::Context::new();
    let mut renderer = inochi2d::gl::setup(&gl, &model.puppet.nodes, textures, width, height);

    while !window.should_close() {
        renderer.clear();
        let order = inochi2d::gl::sort_nodes_by_zsort(&model.puppet.nodes);
        renderer.render_nodes(&order);
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    renderer.set_size(width, height);
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
