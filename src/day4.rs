type SectionInterval = (u32, u32);

pub struct Pair {
    interval1: SectionInterval,
    interval2: SectionInterval,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let (interval1, interval2) = l.split_once(',').unwrap();
            let interval1 = interval1.split_once('-').unwrap();
            let interval1 = (
                interval1.0.parse::<u32>().unwrap(),
                interval1.1.parse::<u32>().unwrap(),
            );
            let interval2 = interval2.split_once('-').unwrap();
            let interval2 = (
                interval2.0.parse::<u32>().unwrap(),
                interval2.1.parse::<u32>().unwrap(),
            );
            Pair {
                interval1,
                interval2,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Pair]) -> u32 {
    input
        .iter()
        .filter(|p| {
            (p.interval1.0 >= p.interval2.0 && p.interval1.1 <= p.interval2.1)
                || (p.interval2.0 >= p.interval1.0 && p.interval2.1 <= p.interval1.1)
        })
        .count() as u32
}

#[aoc(day4, part2)]
pub fn part2(input: &[Pair]) -> u32 {
    input
        .iter()
        .filter(|p| p.interval1.0 <= p.interval2.1 && p.interval1.1 >= p.interval2.0)
        .count() as u32
}
