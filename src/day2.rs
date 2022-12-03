#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().fold(0, |points, round| match round {
        "A X" => points + 1 + 3,
        "A Y" => points + 2 + 6,
        "A Z" => points + 3 + 0,
        "B X" => points + 1 + 0,
        "B Y" => points + 2 + 3,
        "B Z" => points + 3 + 6,
        "C X" => points + 1 + 6,
        "C Y" => points + 2 + 0,
        "C Z" => points + 3 + 3,
        _ => unreachable!(),
    })
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().fold(0, |points, round| match round {
        "A X" => points + 3 + 0,
        "A Y" => points + 1 + 3,
        "A Z" => points + 2 + 6,
        "B X" => points + 1 + 0,
        "B Y" => points + 2 + 3,
        "B Z" => points + 3 + 6,
        "C X" => points + 2 + 0,
        "C Y" => points + 3 + 3,
        "C Z" => points + 1 + 6,
        _ => unreachable!(),
    })
}
