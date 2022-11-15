use std::fs::File;
use std::io::{BufReader, Read};

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
    let data = {
        let file = File::open(&args[1]).unwrap();
        let mut file = BufReader::new(file);
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        data
    };
    let model = inochi2d::Model::parse(&data).unwrap().1;
    let puppet = model.puppet;
    let root = &puppet.nodes;
    println!("{root:?}");
    //visit_node(&puppet.param, root);
}
