use std::fs;

pub fn part_1() -> i32 {
    fs::read_to_string("./inputs/day_01.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .filter_map(|x| {
            let mut digits = x.chars().filter(|c| c.is_digit(10));
            let first_digit = digits.next()?.to_digit(10)? as i32;
            let last_digit = digits.last().unwrap_or(first_digit) as i32;

            Some(first_digit * 10 + last_digit)
        })
        .sum()
}

pub fn part_2() -> i32 {
    0
}
