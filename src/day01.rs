pub fn part_a(input: String) -> i32 {
    input
        .chars()
        .fold(0i32, |acc, c| acc + if c == '(' { 1 } else { -1 })
}

pub fn part_b(input: String) -> i32 {
    // println!("{:?}", input.chars());

    32
}
