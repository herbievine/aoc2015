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

pub fn bench_functions(part_a: fn(String) -> i32, part_b: fn(String) -> i32, input: String) -> () {
    println!("running part a:");
    println!("result is {}", part_a(input.clone()));

    println!("running part b:");
    println!("result is {}", part_b(input.clone()));
}

pub fn display_error_and_exit(msg: String) -> () {
    eprintln!("{}", msg);
    process::exit(1);
}
