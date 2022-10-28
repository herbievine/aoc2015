use std::fs::read_to_string;

mod day01;
mod lib;

fn main() {
    let matches = lib::commands().get_matches();
    let day: u16;
    let parts: (fn(String) -> i32, fn(String) -> i32);
    let mut input = String::new();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            day = *sub_matches.get_one::<u16>("day").unwrap();
        }
        _ => unreachable!(),
    }

    match day {
        1 => {
            parts = (day01::part_a, day01::part_b);
            input = read_to_string(format!("./static/day{}{}.txt", day / 10, day % 10)).unwrap();
        }
        x => {
            lib::display_error_and_exit(format!("Day {} is not yet implemented.", x));
            unreachable!();
        }
    }

    lib::bench_functions(parts.0, parts.1, input);
}
