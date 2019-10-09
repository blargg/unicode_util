mod cli;
mod store;

use crate::store::Store;
use clap::*;
use cursive::{
    *,
    event::*,
    views::*,
};
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
        eprintln!("DEBUG MODE: MANY SYMBOLS WILL BE MISSING.");
        eprintln!("This version of the program is intentionally missing most unicode symbols.");
    }

    let arg_matches = cli::app_arguments().get_matches();

    match arg_matches.subcommand() {
        ("set", Some(matches)) => {
            match run_set(matches) {
                Ok(()) => (),
                Err(msg) => eprintln!("{}", msg),
            }
        }
        ("get", Some(matches)) => {
            run_get(matches);
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

fn run_get<'a>(matches: &ArgMatches<'a>) {
    let var_name = matches.value_of("VAR").unwrap();
    let store = match Store::load_file() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error loading Store file.");
            exit(1);
        }
    };
    let val = store.saved.get(&var_name.to_string());
    match val {
        Some(val) => println!("{}", val),
        None => eprintln!("{:?} is not saved.", var_name),
    };
}

fn run_set<'a>(matches: &ArgMatches<'a>) -> std::result::Result<(), String> {
    let unicode_map = mk_map();
    let var_name = matches.value_of("VAR").unwrap();
    let query = matches.value_of("QUERY").unwrap();

    // Modify the regex
    // Case insensitive, and allows leading and trailing characters
    let regex_string = format!("(?i).*{}.*", query);
    let re = Regex::new(regex_string.as_str())
        .map_err(|e| format!( "Regex \"{}\" failed to compile.\n{}", regex_string, e))?;

    let results = unicode_map
        .search(&re)
        .into_stream()
        .into_str_vec()
        .expect("convert keys to utf-8");

    let mut siv = Cursive::termion().map_err(|_| "Could not initialize terminal")?;
    siv.set_theme(tui::theme());
    siv.add_global_callback('q', |s| s.quit());

    let mut list_view = SelectView::new()
        .on_submit(|cursive: &mut Cursive, value: &u64| {
            cursive.set_user_data(*value);
            cursive.quit();
        });

    for (description, v) in results {
        if let Some(character) = from_u64(v) {
            let line = format!("{} = {:04X}, {}", character, v, description);
            list_view.add_item(line, v);
        } else {
            log::warn!("Index number {} could not be decoded to a character", v);
        }
    }

    let list_view = OnEventView::new(list_view)
        .on_pre_event_inner('k', |s, _| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s, _| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });
    let list_view = ScrollView::new(list_view);

    siv.add_fullscreen_layer(list_view);
    siv.run();

    let selection_code = siv.take_user_data::<u64>().ok_or("No character selected")?;
    drop(siv); // restore terminal
    let c: char = from_u64(selection_code).ok_or("Could not parse character")?;
    let mut store = Store::load_file()
        .map_err(|_| format!("Error loading Store file."))?;
    store.saved.insert(var_name.to_string(), c);
    store.save_file()
        .map_err(|_| format!("Error saving Store file."))
}

mod tui {
    use cursive::theme::Theme;
    pub fn theme() -> Theme {
        let mut theme = Theme::default();
        theme.shadow = false;
        theme
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
