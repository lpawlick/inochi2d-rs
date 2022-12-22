use std::fs::File;
use std::io::BufReader;

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
