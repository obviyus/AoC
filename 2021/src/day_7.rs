use std::fs;

pub const TITLE: &str = "--- Day 7: The Treachery of Whales ---";

pub fn part_1() -> i64 {
    let mut crab_positions = fs::read_to_string("inputs/day7")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.split(','))
        .flatten()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    crab_positions.sort_unstable();
    crab_positions
        .iter()
        .map(|n| (n - crab_positions[crab_positions.len() / 2]).abs())
        .sum()
}

pub fn part_2() -> i64 {
    let crab_positions = fs::read_to_string("inputs/day7")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.split(','))
        .flatten()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    crab_positions
        .iter()
        .map(|x| {
            crab_positions
                .clone()
                .iter()
                .map(|y| (x - y).abs())
                .map(|n| n * (n + 1) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap()
}
