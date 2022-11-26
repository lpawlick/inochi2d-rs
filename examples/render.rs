use std::fs::File;
use std::io::BufReader;

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
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
    let mut model = inochi2d::Model::parse(file).unwrap();
    print_info(&model.puppet.meta);
    inochi2d::gl::render(&mut model);
}
