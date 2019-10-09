///! Defines the command line interface and options.
use clap::*;

pub fn app_arguments<'a, 'b>() -> App<'a, 'b> {
    App::new("unicode_util")
        .author("Tom Jankauski, tomjankauski@gmail.com")
        .about("Converts a latex expression for a symbol to a unicode character")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(lookup_subcommand())
        .subcommand(encode_subcommand())
        .subcommand(set_subcommand())
        .subcommand(get_subcommand())
        .subcommand(generate_completions_subcommand())
}

fn get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("get")
        .about("Gets a value saved by set.")
        .arg(
            Arg::with_name("VAR")
            .required(true)
            .help("Variable to retrieve. Can be saved with unicode_util set.")
        )
}

fn set_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("set")
        .about("Search for a value, and save it for later use.")
        .arg(
            Arg::with_name("VAR")
            .required(true)
            .help("Variable name to save the character to. Can be retrieved with unicode_util get.")
        )
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

fn generate_completions_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("generate_completions")
        .about("Generates completion files for some common shell programs")
        .arg(
            Arg::with_name("SHELL")
            .required(true)
            .possible_values(&["bash", "zsh", "fish", "powershell"])
            .help("Specifies the shell program to generate completions for.")
        )
}
