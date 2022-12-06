use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().collect();
    for (iteration, window) in chars.windows(4).enumerate() {
        let unique = HashSet::<_>::from_iter(window.iter());
        if unique.len() == 4 {
            return (iteration + 4) as u32;
        }
    }
    return 0;
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u32 {
    let chars: Vec<_> = input.chars().collect();
    for (iteration, window) in chars.windows(14).enumerate() {
        let unique = HashSet::<_>::from_iter(window.iter());
        if unique.len() == 14 {
            return (iteration + 14) as u32;
        }
    }
    return 0;
}
