use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|rucksack| {
            let (first, second) = rucksack.split_at(rucksack.len() / 2);
            let item_types_first: HashSet<_> = first.chars().collect();
            second
                .chars()
                .find(|item_type| item_types_first.contains(item_type))
                .unwrap()
        })
        .fold(0, |sum, item_type| match item_type {
            'a'..='z' => sum + item_type as i32 - 'a' as i32 + 1,
            'A'..='Z' => sum + item_type as i32 - 'A' as i32 + 27,
            _ => unreachable!(),
        })
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut item_types_first: HashSet<_> = HashSet::new();
    let mut item_types_second: HashSet<_> = HashSet::new();
    let mut sum = 0;
    for (index, rucksack) in input.lines().enumerate() {
        match index % 3 {
            0 => item_types_first = rucksack.chars().collect(),
            1 => item_types_second = rucksack.chars().collect(),
            2 => {
                let intersection: HashSet<_> =
                    item_types_first.intersection(&item_types_second).collect();
                let badge = rucksack
                    .chars()
                    .find(|item_type| intersection.contains(item_type))
                    .unwrap();
                sum += match badge {
                    'a'..='z' => badge as i32 - 'a' as i32 + 1,
                    'A'..='Z' => badge as i32 - 'A' as i32 + 27,
                    _ => unreachable!(),
                };
                item_types_first = HashSet::new();
                item_types_second = HashSet::new();
            }
            _ => unreachable!(),
        }
    }
    sum
}
