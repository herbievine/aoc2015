use super::PartReturnType;

pub fn get_metadata() -> (String, String) {
    let question_a = String::from("To what floor do the instructions take Santa?");
    let question_b = String::from(
        "What is the position of the character that causes Santa to first enter the basement?",
    );

    (question_a, question_b)
}

pub fn part_a(input: String) -> PartReturnType {
    let res = input
        .chars()
        .fold(0i32, |acc, c| acc + if c == '(' { 1 } else { -1 });

    PartReturnType::I32(res)
}

pub fn part_b(input: String) -> PartReturnType {
    let mut index = 0;

    input.chars().enumerate().fold(0i32, |acc, (i, c)| {
        if acc < 0 && index == 0 {
            index = i as i32;
        };

        acc + if c == '(' { 1 } else { -1 }
    });

    PartReturnType::I32(index)
}
