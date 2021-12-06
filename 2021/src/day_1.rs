pub const TITLE: &str = "Day 1: This is a dummy day.";

use std::fs;

pub fn part_1() -> i64 {
    fs::read_to_string("inputs/day1")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count() as i64
}

pub fn part_2() -> i64 {
    fs::read_to_string("input/day2")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .filter(|x| x[1] + x[2] + x[3] > x[0] + x[1] + x[2])
        .count() as i64
}
