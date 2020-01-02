mod cli;
mod store;
mod tui;

use crate::{
    store::Store,
    tui::character_search,
};
use clap::*;
use fst::*;
use std::{
    char::from_u32,
    convert::TryFrom,
    process::exit,
};

static FST: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/map.fst"));

fn main() {
    match main_err() {
        Err(s) => {
            eprintln!("{}", s);
            exit(1);
        }
        Ok(()) => {}
    }
}

fn main_err() -> MainResult<()> {
    let arg_matches = cli::app_arguments().get_matches();

    match arg_matches.subcommand() {
        ("search", Some(matches)) => {
            run_search(matches)?;
        }
        ("get", Some(matches)) => {
            run_get(matches)?;
        }
        ("lookup", Some(matches)) => {
            run_lookup(matches)?;
        }
        ("encode", Some(matches)) => {
            run_encode(matches)?;
        }
        ("generate_completions", Some(matches)) => {
            run_generate_completions(matches)?;
        }
        (cmd, _) => {
            return Err(format!("Command \"{}\" not found", cmd));
        }
    }

    Ok(())
}

/// Makes a new copy of the map
fn mk_map() -> Map {
    Map::from_static_slice(FST).unwrap()
}

fn run_get<'a>(matches: &ArgMatches<'a>) -> MainResult<()> {
    let var_name = matches.value_of("VAR").unwrap();
    let store = Store::load_file()
        .map_err(|_| format!("Error loading Store file."))?;
    let val = store.saved.get(&var_name.to_string())
        .ok_or(format!("{:?} is not saved.", var_name))?;
    println!("{}", val);

    Ok(())
}

fn run_search<'a>(matches: &ArgMatches<'a>) -> MainResult<()> {
    let query = matches.value_of("QUERY").unwrap();
    let results = tui::search(query);
    let mut siv = tui::initialize_cursive().ok_or("Could not initialize terminal")?;
    let list_view = character_search(results.into_iter());
    siv.add_fullscreen_layer(list_view);
    siv.run();
    Ok(())
}

fn run_lookup<'a>(matches: &ArgMatches<'a>) -> MainResult<()> {
    let code = matches.value_of("CODE").unwrap();
    let c = parse_hex_str(code)
        .ok_or(format!("Could not parse \"{}\" into a character", code))?;
    println!("{}", c);
    Ok(())
}

fn run_encode<'a>(matches: &ArgMatches<'a>) -> MainResult<()> {
    let char_str = matches.value_of("CHARACTER").unwrap();
    let c = char_str.chars().next()
        .ok_or(format!("Encountered empty string"))?;
    println!("{:04X}", c as u32);
    Ok(())
}

/// Generates completion functions for the given shell.
fn run_generate_completions<'a>(matches: &ArgMatches<'a>) -> MainResult<()> {
    let shell_string = matches.value_of("SHELL").unwrap();

    use clap::Shell::*;
    let shell_type = match shell_string {
        "bash" => Bash,
        "zsh" => Zsh,
        "fish" => Fish,
        "powershell" => PowerShell,
        other => {
            return Err(format!("{} shell not supported.", other))
        }
    };
    cli::app_arguments().gen_completions_to("unicode_util", shell_type, &mut std::io::stdout());
    Ok(())
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

type MainResult<A> = std::result::Result<A, String>;
