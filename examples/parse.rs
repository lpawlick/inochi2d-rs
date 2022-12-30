// Copyright (c) 2022 Emmanuel Gil Peyrot <linkmauve@linkmauve.fr>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::BufReader;

/*
fn visit_node(param: &[inochi2d::parser::Param], node: &inochi2d::Node) {
    println!("{:?}: {:?}", node, node.param(param));
    for child in node.children.iter() {
        visit_node(param, child);
    }
}
*/

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <model.inp>", args[0]);
        return;
    }
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
    let model = inochi2d::Model::parse(file).unwrap();
    let puppet = model.puppet;
    let root = &puppet.nodes;
    println!("{root:?}");
    //visit_node(&puppet.param, root);
}
