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

fn unique_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .filter(|&each| !s2.contains(each))
        .collect::<String>()
}

fn convert(input: Vec<&str>, output: Vec<&str>) -> i64 {
    // Sort each string in the vector
    let sorted_output: Vec<String> = output
        .iter()
        .map(|&each| {
            let mut chars = each.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| a.cmp(b));

            String::from_iter(chars)
        })
        .collect();

    let mut input_sorted_by_length: Vec<&str> = input.clone();
    input_sorted_by_length.sort_unstable_by(|a, b| a.len().cmp(&b.len()));

    let mut mappings = vec![""; 10];
    mappings[1] = input_sorted_by_length[0];
    mappings[4] = input_sorted_by_length[2];
    mappings[7] = input_sorted_by_length[1];
    mappings[8] = input_sorted_by_length[9];

    let mut top_right = "".to_string();
    let mut bottom_right = "".to_string();

    for i in 6..9 {
        if unique_chars(input_sorted_by_length[i], input_sorted_by_length[0].clone()).len() == 5 {
            mappings[6] = input_sorted_by_length[i];
            top_right = unique_chars(input_sorted_by_length[9], mappings[6].clone())
                .chars()
                .next()
                .unwrap()
                .to_string();
            bottom_right = unique_chars(mappings[1], top_right.as_str())
                .chars()
                .next()
                .unwrap()
                .to_string();
            break;
        }
    }

    for i in 3..6 {
        if input_sorted_by_length[i].contains(&top_right)
            && input_sorted_by_length[i].contains(&bottom_right)
        {
            mappings[3] = input_sorted_by_length[i];
        } else if input_sorted_by_length[i].contains(&top_right) {
            mappings[2] = input_sorted_by_length[i];
        } else {
            mappings[5] = input_sorted_by_length[i];
        }
    }

    for i in 6..9 {
        if input_sorted_by_length[i] == mappings[6] {
            continue;
        } else {
            if unique_chars(input_sorted_by_length[i], mappings[4].clone()).len() == 2 {
                mappings[9] = input_sorted_by_length[i];
            } else {
                mappings[0] = input_sorted_by_length[i];
            }
        }
    }

    let mut actual_output = "".to_string();
    let sorted_mappings: Vec<String> = mappings
        .iter()
        .map(|&each| {
            let mut chars = each.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| a.cmp(b));

            String::from_iter(chars)
        })
        .collect();

    for digit in sorted_output {
        actual_output.push_str(
            sorted_mappings
                .iter()
                .position(|each| each.as_str() == digit)
                .unwrap()
                .to_string()
                .as_str(),
        );
    }

    actual_output.parse::<i64>().unwrap()
}

pub fn part_2() -> i64 {
    fs::read_to_string("inputs/day8")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            let mut pair = line.split(" | ");
            let input = pair.next().unwrap().split(" ").collect::<Vec<&str>>();
            let output = pair.next().unwrap().split(" ").collect::<Vec<&str>>();
            convert(input, output)
        })
        .sum()
}
