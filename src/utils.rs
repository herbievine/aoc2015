use crate::days::Part;
use std::process;

pub fn run_day(day_num: u16, day: (Part, Part), input: String) -> () {
    println!(
        "---------------  Running day {}  ---------------\n",
        day_num
    );

    run_part(day.0.part_char, day.0, input.clone());
    run_part(day.1.part_char, day.1, input.clone());

    println!("----------------  Ended day {}  ----------------", day_num);
}

fn run_part(part_char: char, part: Part, input: String) -> () {
    println!("---------------");

    println!("Part {}: {}", part_char, part.question);
    println!("Result: {}", (part.function)(input));

    println!("---------------\n");
}

#[warn(unreachable_code)]
pub fn display_error_and_exit(msg: String) -> () {
    eprintln!("{}", msg);
    process::exit(1);
}
