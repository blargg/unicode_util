use fst::*;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::env;
use std::{
    char::from_u32,
    convert::TryFrom,
};
use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, XmlEvent},
};

fn main() {
    let tags = parse_file().unwrap();
    for (_tag, _) in tags {
    }
}

fn from_u64(n: u64) -> Option<char> {
    u32::try_from(n)
        .ok()
        .and_then(|n32| from_u32(n32))
}

fn to_u64(c: char) -> u64 {
    c as u64
}

fn write_unicode_map() -> fst::Result<()> {
    let mut associations = parse_file().unwrap();
    associations.sort_by(|x, y| x.0.cmp(&y.0));
    associations.dedup_by_key(|x| x.0.clone());

    let out_dir = env::var("OUT_DIR").unwrap();
    let fst_path = Path::new(&out_dir).join("map.fst");
    println!("{:?}", fst_path);

    let writer = io::BufWriter::new(File::create(fst_path).unwrap());
    let mut build = MapBuilder::new(writer)?;

    for (key, ch) in associations.iter() {
        build.insert(key, *ch)?;
    }

    build.finish().unwrap();
    Ok(())
}

/// Reads the file, returns a string representing the latex string, and the unicode symbol
fn parse_file() -> io::Result<Vec<(String, u64)>> {
    // number base of the file's character encoding
    const BASE: u32 = 16;
    let file = File::open("data/ucd.all.flat.xml")?;
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let mut associations = Vec::new();
    for e in parser.into_iter().take(1050) { // TODO remove
        match e {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if name.local_name == "char" {
                    let code = get_attr(&attributes, "cp");
                    let c = code
                        .and_then(|code| u64::from_str_radix(code.as_str(), BASE).ok());
                    if let Some(character) = c {
                        associations.push((name.local_name, character));
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            _ => {}
        }
    }

    Ok(associations)
}

fn get_attr(attrs: &Vec<OwnedAttribute>, attr_name: &str) -> Option<String> {
    attrs
        .into_iter()
        .find(|oa| oa.name.local_name == attr_name)
        .map(|oa| oa.value.clone())
}
