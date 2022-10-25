use clap::{value_parser, Arg, Command};
use std::process;

pub fn commands() -> Command {
    let day_arg = Arg::new("day")
        .value_parser(value_parser!(u16))
        .required(true);

    Command::new("aoc")
        .about("A CLI to run Advent of Code projects.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("Run all or a specific project.")
                .arg(day_arg)
                .arg_required_else_help(true),
        )
}

pub fn display_error_and_exit(msg: String) -> () {
    eprintln!("{}", msg);
    process::exit(1);
}
