#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .fold(0, |sum, food| sum + food.parse::<i32>().unwrap())
        })
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut inventories: Vec<_> = input
        .split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .fold(0, |sum, food| sum + food.parse::<i32>().unwrap())
        })
        .collect();
    inventories.sort();
    inventories
        .iter()
        .rev()
        .take(3)
        .fold(0, |sum, calories| sum + calories)
}
