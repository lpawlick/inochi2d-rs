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

    inochi2d::gl::render(&mut model, width, height);
}
