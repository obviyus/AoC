use std::{collections::BinaryHeap, fs};

pub fn part_1() -> i32 {
    let input: BinaryHeap<i32> = fs::read_to_string("../inputs/day_01.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    input.peek().unwrap().clone()
}

pub fn part_2() -> i32 {
    let input: BinaryHeap<i32> = fs::read_to_string("../inputs/day_01.txt")
        .unwrap()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    input.iter().take(3).sum()
}
