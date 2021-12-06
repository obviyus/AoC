use std::{collections::HashMap, fs};

pub const TITLE: &str = "Day 6: This is a dummy day.";

fn runner(fishes: HashMap<i64, i64>, days: i64) -> i64 {
    let mut counter = fishes.clone();

    for _ in 0..days {
        let mut today: HashMap<i64, i64> = HashMap::new();
        for (k, v) in counter.iter() {
            match k {
                0 => {
                    *today.entry(6).or_insert(0) += v;
                    *today.entry(8).or_insert(0) += v;
                }
                _ => {
                    *today.entry(k - 1).or_insert(0) += v;
                }
            }
        }
        counter = today.clone();
    }

    counter.values().sum()
}

pub fn part_1() -> i64 {
    let fishes: HashMap<i64, i64> = fs::read_to_string("inputs/day6")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    runner(fishes, 80)
}

pub fn part_2() -> i64 {
    let fishes: HashMap<i64, i64> = fs::read_to_string("inputs/day6")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    runner(fishes, 256)
}
