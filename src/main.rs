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
    let unicode_map = mk_map();
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
            let c: char = from_u64(*value)
                .expect( "Could not parse character");
            cursive.add_layer(tui::save_prompt(c));
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
    Ok(())
}

mod tui {
    use cursive::{
        Cursive,
        theme::Theme,
        views::*,
        view::*,
    };
    use crate::store::Store;

    pub fn theme() -> Theme {
        let mut theme = Theme::default();
        theme.shadow = false;
        theme
    }

    pub fn save_prompt(val_to_save: char) -> Dialog {
        Dialog::new()
            .title("Save to")
            .padding((1, 1, 1, 0))
            .content(
                EditView::new()
                    .on_submit(move |cursive, var_name| save(cursive, var_name, val_to_save))
                    .with_id("name")
                    .fixed_width(20),
            )
            .button("Ok", move |s| {
                let name = s.call_on_id(
                    "name",
                    |view: &mut EditView| view.get_content(),
                ).unwrap();
                save(s, &name, val_to_save);
            })
    }

    fn save(s: &mut Cursive, var_name: &str, value: char) {
        if var_name.is_empty() {
            s.add_layer(Dialog::info("Enter a var name"));
        } else {
            let mut store = Store::load_file()
                .expect("Error loading Store file.");
            store.saved.insert(var_name.to_string(), value);
            store.save_file()
                .expect("Error saving Store file.");

            s.quit();
        }
    }
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
