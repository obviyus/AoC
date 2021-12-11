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

    let middle = crab_positions.len() / 2;
    let median = *crab_positions.select_nth_unstable(middle).1;

    crab_positions.iter().map(|n| (n - median).abs()).sum()
}

fn cost(n: i64) -> i64 {
    n * (n + 1) / 2
}

pub fn part_2() -> i64 {
    let crab_positions = fs::read_to_string("inputs/day7")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.split(','))
        .flatten()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (crab_positions.iter().sum::<i64>() / crab_positions.len() as i64..)
        .take(2)
        .map(|t| {
            crab_positions
                .iter()
                .map(|n| cost((n - t).abs()))
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 335271);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 95851339);
}
