use std::fs;

const PART_1_WINDOW_LENGTH: usize = 4;
const PART_2_WINDOW_LENGTH: usize = 14;

fn solver(input: Vec<char>, window_length: usize) -> i32 {
    input
        .windows(window_length)
        .position(|window| {
            let mut current_window = window.to_vec();
            current_window.sort();
            current_window.dedup();
            current_window.len() == window_length
        })
        .expect("No valid marker found") as i32
        + window_length as i32
}

pub fn part_1() -> i32 {
    let input = fs::read_to_string("../inputs/day_06.txt")
        .expect("Error reading input file")
        .chars()
        .collect::<Vec<char>>();

    solver(input, PART_1_WINDOW_LENGTH)
}

pub fn part_2() -> i32 {
    let input = fs::read_to_string("../inputs/day_06.txt")
        .expect("Error reading input file")
        .chars()
        .collect::<Vec<char>>();

    solver(input, PART_2_WINDOW_LENGTH)
}
