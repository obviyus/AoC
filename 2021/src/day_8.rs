use std::fs;

pub const TITLE: &str = "--- Day 8: Seven Segment Search ---";

pub fn part_1() -> i64 {
    fs::read_to_string("inputs/day8")
        .expect("Unable to read input")
        .lines()
        .map(|line| line.split(" | ").skip(1).next().unwrap())
        .map(|output| output.split(" "))
        .flatten()
        .filter(|&each| match each.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count() as i64
}

fn intersection(s1: &str, s2: &str) -> String {
    s1.chars()
        .filter(|&each| s2.contains(each))
        .collect::<String>()
}

fn convert(input: Vec<&str>, output: Vec<&str>) -> i64 {
    let mut input_sorted_by_length: Vec<&str> = input.clone();
    input_sorted_by_length.sort_unstable_by(|&a, &b| a.len().cmp(&b.len()));

    let mut mappings: Vec<&str> = vec![""; 10];
    mappings[1] = &input_sorted_by_length[0];
    mappings[4] = &input_sorted_by_length[2];
    mappings[7] = &input_sorted_by_length[1];
    mappings[8] = &input_sorted_by_length[9];

    for &length_five in &input_sorted_by_length[3..6] {
        if mappings[7].chars().all(|char| length_five.contains(char)) {
            mappings[3] = length_five;
        } else {
            if intersection(length_five, &mappings[4]).len() == 3 {
                mappings[5] = length_five;
            } else {
                mappings[2] = length_five;
            }
        }
    }

    for &length_six in &input_sorted_by_length[6..9] {
        if mappings[4].chars().all(|char| length_six.contains(char)) {
            mappings[9] = length_six;
        } else {
            if mappings[7].chars().all(|char| length_six.contains(char)) {
                mappings[0] = length_six;
            } else {
                mappings[6] = length_six;
            }
        }
    }

    let mut actual_output = Vec::with_capacity(4);
    for digit in output {
        actual_output.push(
            mappings
                .iter()
                .position(|&each| each == digit)
                .unwrap()
                .to_string(),
        );
    }

    String::from_iter(actual_output).parse::<i64>().unwrap()
}

pub fn part_2() -> i64 {
    let result: i64 = fs::read_to_string("inputs/day8")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            let mut pair = line.split(" | ");
            let input = pair
                .next()
                .unwrap()
                .split(" ")
                .map(|each| {
                    let mut chars = each.chars().collect::<Vec<char>>();
                    chars.sort_by(|a, b| a.cmp(b));
                    String::from_iter(chars)
                })
                .collect::<Vec<String>>();
            let input = input
                .iter()
                .map(|each| each.as_str())
                .collect::<Vec<&str>>();

            let output = pair
                .next()
                .unwrap()
                .split(" ")
                .map(|each| {
                    let mut chars = each.chars().collect::<Vec<char>>();
                    chars.sort_by(|a, b| a.cmp(b));
                    String::from_iter(chars)
                })
                .collect::<Vec<String>>();
            let output = output
                .iter()
                .map(|each| each.as_str())
                .collect::<Vec<&str>>();

            convert(input, output)
        })
        .sum();

    result
}
