use std::fs;

pub fn part_1() -> usize {
    fs::read_to_string("day1/input")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
}

pub fn part_2() -> usize {
    fs::read_to_string("day1/input")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .filter(|x| x[1] + x[2] + x[3] > x[0] + x[1] + x[2])
        .count()
}
