use std::fs::File;
use std::io::{BufReader, Read};

fn print_info(meta: &inochi2d::Meta) {
    if let Some(ref name) = meta.name {
        println!("Model {name}");
    }
    println!("Version {}", meta.version);
    println!("Rigger {}", meta.rigger);
    println!("Artist {}", meta.artist);
    if let Some(ref rights) = meta.rights {
        println!("Rights {rights}");
    }
    println!("Copyright {}", meta.copyright);
    println!("License {}", meta.license_url);
    println!("Contact {}", meta.contact);
    if let Some(ref reference) = meta.reference {
        println!("Reference {reference}");
    }
}

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
    let mut model = inochi2d::Model::parse(&data).unwrap().1;
    print_info(&model.puppet.meta);
    inochi2d::gl::render(&mut model);
}
