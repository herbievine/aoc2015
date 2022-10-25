mod day01;
mod lib;

fn main() {
    let matches = lib::commands().get_matches();
    let day: u16;

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            day = *sub_matches.get_one::<u16>("day").unwrap();
        }
        _ => unreachable!(),
    }

    match day {
        1 => day01::solve(true),
        x => lib::display_error_and_exit(format!("Day {} is not yet implemented.", x)),
    }
}
