use fst::*;
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    process::Command,
    env,
};
use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, XmlEvent},
};

fn main() {
    cargo_settings();
    let zip_path = "data.tmp/ucd.all.flat.zip";
    let xml_path = "data.tmp/ucd.all.flat.xml";
    if !Path::new(xml_path).is_file() {
        download_ucd_all(zip_path);
        unzip(zip_path);
    }
    write_unicode_map("data.tmp/ucd.all.flat.xml").unwrap();
}

fn cargo_settings() {
    println!("cargo:rerun-if-changed=build.rs");
}

fn write_unicode_map(ucd_xml_path: &str) -> fst::Result<()> {
    let mut associations = parse_file(ucd_xml_path).unwrap();
    associations.sort_by(|x, y| x.0.cmp(&y.0));
    associations.dedup_by_key(|x| x.0.clone());

    let out_dir = env::var("OUT_DIR").unwrap();
    let fst_path = Path::new(&out_dir).join("map.fst");
    println!("output_path = {:?}", fst_path);

    let writer = io::BufWriter::new(File::create(fst_path).unwrap());
    let mut build = MapBuilder::new(writer)?;

    for (key, ch) in associations.iter() {
        build.insert(key, *ch)?;
    }

    build.finish().unwrap();
    Ok(())
}

/// Downloads the unicode files if they are not already stored on disk
/// Creates the file in data.tmp directory. If the file already exits, this function will make no
/// changes.
fn download_ucd_all(dest_path: &str) {
    let url = "http://www.unicode.org/Public/12.1.0/ucdxml/ucd.all.flat.zip";
    if !Path::new(dest_path).is_file() {
        println!("{} is not cached, downloading", dest_path);
        let mut cmd = Command::new("curl");
        cmd
            .arg("-vs")
            .arg(format!("-o{}", dest_path))
            .arg("--create-dirs")
            .arg(url);
        println!("curl command: {:?}", cmd);
        let status = cmd
            .status()
            .expect("failed to download unicode data");
        assert!(status.success(), "curl exited with a non zero status");
    }
}

const DATA_TMP_DIR: &'static str = "data.tmp";

/// runs the unzip command on the given file
fn unzip(path: &str) {
    let status = Command::new("unzip")
        .arg(format!("-d{}", DATA_TMP_DIR))
        .arg(path)
        .status()
        .expect("failed to run unzip");
    assert!(status.success(), "unzip exited with non zero status");
}

/// Reads the file, returns a string representing the latex string, and the unicode symbol
fn parse_file(ucd_xml_path: &str) -> io::Result<Vec<(String, u64)>> {
    // number base of the file's character encoding
    const BASE: u32 = 16;
    let file = File::open(ucd_xml_path)?;
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let mut associations = Vec::new();

    for e in parser.into_iter() {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if name.local_name == "char" {
                    // TODO support characters with aliases
                    if let Some(desc) = get_attr(&attributes, "na") {
                        let code = get_attr(&attributes, "cp");
                        let c = code
                            .and_then(|code| u64::from_str_radix(code.as_str(), BASE).ok());
                        if let Some(character) = c {
                            associations.push((desc, character));
                        }
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
