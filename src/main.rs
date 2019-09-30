use fst::*;
use fst_regex::Regex;
use std::{
    char::from_u32,
    convert::TryFrom,
};

static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/map.fst"));

fn main() {
    let unicode_map = Map::from_static_slice(FST).unwrap();

    let regex_string = ".*LATIN.*".to_owned();
    let re = match Regex::new(regex_string.as_str()) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Regex \"{}\" failed to compile.", regex_string);
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let matches = unicode_map
        .search(&re)
        .into_stream()
        .into_str_vec()
        .expect("convert keys to utf-8");
    for (k, v) in matches {
        if let Some(character) = from_u64(v) {
            println!("{} = {}", character, k);
        } else {
            eprintln!("could not print character");
        }
    }
}

fn from_u64(n: u64) -> Option<char> {
    u32::try_from(n)
        .ok()
        .and_then(|n32| from_u32(n32))
}
