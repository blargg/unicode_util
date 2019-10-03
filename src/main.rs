use clap::*;
use fst::*;
use fst_regex::Regex;
use std::{
    char::from_u32,
    convert::TryFrom,
    process::exit,
};

mod cli;

static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/map.fst"));

fn main() {
    #[cfg(debug_assertions)]
    {
        println!("DEBUG MODE MANY SYMBOLS WILL BE MISSING");
        println!("This version of the program is intentionally missing most unicode symbols.");
    }

    let arg_matches = cli::app_arguments().get_matches();

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
        ("generate_completions", Some(matches)) => {
            run_generate_completions(matches);
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

/// Generates completion functions for the given shell.
fn run_generate_completions<'a>(matches: &ArgMatches<'a>) {
    let shell_string = matches.value_of("SHELL").unwrap();

    use clap::Shell::*;
    let shell_type = match shell_string {
        "bash" => Bash,
        "zsh" => Zsh,
        "fish" => Fish,
        "powershell" => PowerShell,
        other => {
            eprintln!("{} shell not supported.", other);
            exit(1);
        }
    };
    cli::app_arguments().gen_completions_to("unicode_util", shell_type, &mut std::io::stdout());
}

fn parse_hex_str(s: &str) -> Option<char> {
    let n = u64::from_str_radix(s, 16).ok()?;
    from_u64(n)
}


fn from_u64(n: u64) -> Option<char> {
    u32::try_from(n)
        .ok()
        .and_then(|n32| from_u32(n32))
}
