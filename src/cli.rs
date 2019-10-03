///! Defines the command line interface and options.
use clap::*;

pub fn app_arguments<'a, 'b>() -> App<'a, 'b> {
    App::new("unicode_tex")
        .author("Tom Jankauski, tomjankauski@gmail.com")
        .about("Converts a latex expression for a symbol to a unicode character")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(seach_subcommand())
        .subcommand(lookup_subcommand())
        .subcommand(encode_subcommand())
        .subcommand(generate_completions_subcommand())
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
