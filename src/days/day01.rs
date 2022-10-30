pub fn get_metadata() -> (String, String) {
    let question_a = String::from("To what floor do the instructions take Santa?");
    let question_b = String::from(
        "What is the position of the character that causes Santa to first enter the basement?",
    );

    (question_a, question_b)
}

pub fn part_a(input: String) -> i32 {
    input
        .chars()
        .fold(0i32, |acc, c| acc + if c == '(' { 1 } else { -1 })
}

pub fn part_b(_input: String) -> i32 {
    32
}
