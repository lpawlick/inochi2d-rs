// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <PNG|TGA> <model.inp> <out.inp>", args[0]);
        return;
    }

    let format = match args[1].as_str() {
        "PNG" => image::ImageFormat::Png,
        "TGA" => image::ImageFormat::Tga,
        _ => panic!("Unsupported format {}", args[1]),
    };
    let file = File::open(&args[2]).unwrap();
    let file = BufReader::new(file);
    let mut model = inochi2d::Model::parse(file).unwrap();

    for tex in model.textures.iter_mut() {
        tex.decode();
        tex.encode(format);
    }

    let out = File::create(&args[3]).unwrap();
    model.serialize(out).unwrap();
}
