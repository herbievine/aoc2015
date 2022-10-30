use std::fs::read_to_string;

mod cli;
mod days;
mod utils;

fn main() {
    let commands = cli::commands();
    let day: u16;
    let input: String;

    match commands.get_matches().subcommand() {
        Some(("run", sub_matches)) => {
            day = *sub_matches.get_one::<u16>("day").unwrap();
        }
        _ => unreachable!(),
    }

    match day {
        1 => {
            input = read_to_string(format!("./static/day{}{}.txt", day / 10, day % 10)).unwrap();
            utils::run_day(day, days::get_day(1), input);
        }
        x => {
            utils::display_error_and_exit(format!("Day {} is not yet implemented.", x));
        }
    }
}
