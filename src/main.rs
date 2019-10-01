use clap::*;
use fst::*;
use fst_regex::Regex;
use std::{
    char::from_u32,
    convert::TryFrom,
    process::exit,
};

static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/map.fst"));

fn main() {
    #[cfg(debug_assertions)]
    {
        println!("DEBUG MODE MANY SYMBOLS WILL BE MISSING");
        println!("This version of the program is intentionally missing most unicode symbols.");
    }

    let arg_matches = arg::app_arguments().get_matches();

    match arg_matches.subcommand() {
        ("search", Some(matches)) => {
            run_search(matches);
        }
        ("lookup", Some(matches)) => {
            run_lookup(matches);
        }
        ("encode", Some(matches)) => {
            run_encode(matches);
        }
        (cmd, _) => {
            eprintln!("Command \"{}\" not found", cmd);
        }
    }
}

/// Makes a new copy of the map
fn mk_map() -> Map {
    Map::from_static_slice(FST).unwrap()
}

fn run_search<'a>(matches: &ArgMatches<'a>) {
    let unicode_map = mk_map();
    let query = matches.value_of("QUERY").unwrap();

    // Modify the regex
    // Case insensitive, and allows leading and trailing characters
    let regex_string = format!("(?i).*{}.*", query);
    let re = match Regex::new(regex_string.as_str()) {
        Ok(re) => re,
        Err(e) => {
            eprintln!("Regex \"{}\" failed to compile.", regex_string);
            eprintln!("{}", e);
            exit(1);
        }
    };
    let results = unicode_map
        .search(&re)
        .into_stream()
        .into_str_vec()
        .expect("convert keys to utf-8");
    for (description, v) in results {
        if let Some(character) = from_u64(v) {
            println!("{} = {:04X}, {}", character, v, description);
        } else {
            eprintln!("could not print character");
            exit(1);
        }
    }
}

fn run_lookup<'a>(matches: &ArgMatches<'a>) {
    let code = matches.value_of("CODE").unwrap();
    if let Some(c) = parse_hex_str(code) {
        println!("{}", c);
    } else {
        eprintln!("Could not parse \"{}\" into a character", code);
        exit(1);
    }
}

fn run_encode<'a>(matches: &ArgMatches<'a>) {
    let char_str = matches.value_of("CHARACTER").unwrap();
    if let Some(c) = char_str.chars().next() {
        println!("{:04X}", c as u32);
    } else {
        eprintln!("Encountered empty string");
        exit(1);
    }
}

fn parse_hex_str(s: &str) -> Option<char> {
    let n = u64::from_str_radix(s, 16).ok()?;
    from_u64(n)
}

/// module for the name space of constant strings used for arguments
mod arg {
    use clap::*;

    pub fn app_arguments<'a, 'b>() -> App<'a, 'b> {
        App::new("unicode_tex")
            .author("Tom Jankauski, tomjankauski@gmail.com")
            .about("Converts a latex expression for a symbol to a unicode character")
            .version(env!("CARGO_PKG_VERSION"))
            .subcommand(seach_subcommand())
            .subcommand(lookup_subcommand())
            .subcommand(encode_subcommand())
    }

    fn seach_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("search")
            .about("search character descriptions")
            .arg(
                Arg::with_name("QUERY")
                .required(true)
                .help("search expression")
            )
    }

    fn lookup_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("lookup")
            .about("Converts a character code to an actual character")
            .arg(
                Arg::with_name("CODE")
                .required(true)
                .help("utf-8 character code")
            )
    }

    fn encode_subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("encode")
            .about("Converts a character into a character code")
            .arg(
                Arg::with_name("CHARACTER")
                .required(true)
                .help("utf-8 character")
            )
    }
}

fn from_u64(n: u64) -> Option<char> {
    u32::try_from(n)
        .ok()
        .and_then(|n32| from_u32(n32))
}
