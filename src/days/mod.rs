mod day01;

pub struct Part {
    pub part_char: char,
    pub question: String,
    pub function: fn(String) -> i32,
}

pub fn get_day(day: u16) -> (Part, Part) {
    let metadata: (String, String);
    let part_a: Part;
    let part_b: Part;

    match day {
        1 => {
            metadata = day01::get_metadata();
            part_a = Part {
                part_char: 'A',
                question: metadata.0,
                function: day01::part_a,
            };
            part_b = Part {
                part_char: 'B',
                question: metadata.1,
                function: day01::part_b,
            };
        }
        _ => unreachable!(),
    }

    (part_a, part_b)
}
