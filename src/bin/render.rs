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

    let mut params = Vec::new();
    for param in model.puppet.param {
        println!("{} {:?}", param.name, param.axis_points);
        if [
            "Eye:: Left:: Blink",
            "Eye:: Right:: Blink",
            "Eye:: Left:: Move",
            "Eye:: Right:: Move",
            "Eye:: Left:: Open",
            "Eye:: Right:: Open\0",
            "Eye:: Left:: XY",
            "Eye:: Right:: XY",
            "Eyebrow:: Left",
            "Eyebrow:: Right\0",
            "Breathe",
            "Mouth:: Open / Emotion",
            "Mouth:: Shape",
        ]
        .contains(&param.name.as_str())
        {
            params.push(param);
        }
        params.sort_by(|a, b| a.name.cmp(&b.name));
    }

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
    let mut cur_width = width as f32;
    let mut cur_height = height as f32;

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
    window.set_cursor_pos_polling(true);

    let gl = inochi2d::glow::Context::new();
    let mut renderer = inochi2d::gl::setup(&gl, &model.puppet.nodes, textures, width, height);

    let num_nodes = inochi2d::gl::count_nodes(&model.puppet.nodes);
    let mut params = inochi2d::ParamValues::new(&params);

    let mut frame = 0;
    while !window.should_close() {
        frame += 1;

        let step = 1. - (((frame % 180) as f32) / 20. - 4.5).abs().clamp(0., 1.);
        params.set("Eye:: Left:: Blink", [step, 0.]);
        params.set("Eye:: Right:: Blink", [step, 0.]);
        let step = (((frame % 180) as f32) / 20. - 4.5)
            .abs()
            .clamp(0., 0.8333333333333333);
        params.set("Eye:: Left:: Open", [step, 0.]);
        params.set("Eye:: Right:: Open\0", [step, 0.]);
        let step = (((frame % 240) as f32) / 120. - 1.).abs();
        params.set("Breathe", [step, 0.]);

        renderer.clear();
        renderer.animate(&params);
        let order = inochi2d::gl::sort_nodes_by_zsort(num_nodes, &model.puppet.nodes);
        renderer.render_nodes(&order);
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    cur_width = width as f32;
                    cur_height = height as f32;
                    renderer.set_size(width, height);
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let x = x as f32 / cur_width;
                    let y = y as f32 / cur_height;
                    params.set("Eye:: Left:: Move", [x, 0.]);
                    params.set("Eye:: Right:: Move", [x, 0.]);
                    params.set("Eye:: Left:: XY", [x, 1. - y]);
                    params.set("Eye:: Right:: XY", [x, 1. - y]);
                    params.set("Eyebrow:: Left", [y, 0.]);
                    params.set("Eyebrow:: Right\0", [y, 0.]);
                    params.set("Mouth:: Shape", [x, y]);
                    params.set("Mouth:: Open / Emotion", [x, y]);
                }
                _ => {}
            }
        }
    }
}
