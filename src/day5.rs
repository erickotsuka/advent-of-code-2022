use std::collections::{BTreeMap, LinkedList};

pub struct RearrangementInstruction {
    pub quantity: u32,
    pub from: u32,
    pub to: u32,
}

pub struct RearrangementPlan {
    pub stacks: BTreeMap<u32, LinkedList<char>>,
    pub instructions: Vec<RearrangementInstruction>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> RearrangementPlan {
    let (stacks_str, instructions_str) = input.split_once("\n\n").unwrap();
    let chars: Vec<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let chars = &chars[..];
    let stacks: BTreeMap<u32, LinkedList<char>> = stacks_str
        .lines()
        .flat_map(|l| {
            let crate_locations: Vec<_> = l
                .match_indices(chars)
                .map(|(index, matched)| (((index + 3) / 4) as u32, matched.chars().last().unwrap()))
                .collect();
            crate_locations
        })
        .fold(
            BTreeMap::new(),
            |mut crate_stacks, (stack_id, crate_value)| {
                crate_stacks
                    .entry(stack_id)
                    .and_modify(|crates| crates.push_front(crate_value))
                    .or_insert(LinkedList::from([crate_value]));
                crate_stacks
            },
        );
    let instructions: Vec<_> = instructions_str
        .lines()
        .map(|l| {
            let values: Vec<_> = l
                .split(' ')
                .filter(|word| (*word).parse::<u32>().is_ok())
                .map(|word| (*word).parse::<u32>().unwrap())
                .collect();
            assert!(values.len() == 3);
            RearrangementInstruction {
                quantity: values[0],
                from: values[1],
                to: values[2],
            }
        })
        .collect();
    RearrangementPlan {
        stacks,
        instructions,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &RearrangementPlan) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in input.instructions.iter() {
        for _ in 0..instruction.quantity {
            let moved_crate = stacks
                .get(&instruction.from)
                .unwrap()
                .back()
                .unwrap()
                .clone();
            stacks.entry(instruction.from).and_modify(|crates| {
                crates.pop_back();
            });
            stacks
                .entry(instruction.to)
                .and_modify(|crates| crates.push_back(moved_crate))
                .or_insert(LinkedList::from([moved_crate]));
        }
    }
    let mut tops = String::new();
    for (_, crates) in stacks.iter() {
        tops.push(crates.back().unwrap().clone());
    }
    tops
}

#[aoc(day5, part2)]
pub fn part2(input: &RearrangementPlan) -> String {
    let mut stacks = input.stacks.clone();
    for instruction in input.instructions.iter() {
        let mut moved_crates = LinkedList::new();
        for _ in 0..instruction.quantity {
            moved_crates.push_front(
                stacks
                    .get(&instruction.from)
                    .unwrap()
                    .back()
                    .unwrap()
                    .clone(),
            );
            stacks.entry(instruction.from).and_modify(|crates| {
                crates.pop_back();
            });
        }
        stacks
            .entry(instruction.to)
            .and_modify(|crates| crates.append(&mut moved_crates))
            .or_insert(LinkedList::from(moved_crates));
    }
    let mut tops = String::new();
    for (_, crates) in stacks.iter() {
        tops.push(crates.back().unwrap().clone());
    }
    tops
}
