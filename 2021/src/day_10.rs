pub const TITLE: &str = "--- Day 10: Syntax Scoring ---";

use std::fs;

pub fn part_1() -> i64 {
    let openers: [char; 4] = ['(', '{', '<', '['];
    let closers: [char; 4] = [')', '}', '>', ']'];
    let closer_points: [i64; 4] = [3, 1197, 25137, 57];

    fs::read_to_string("inputs/day10")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut error_score = 0;
            for bracket in line.chars() {
                if openers.contains(&bracket) {
                    stack.push(bracket);
                } else {
                    let score = match stack.pop() {
                        Some(x) => {
                            let opener_index = openers.iter().position(|&o| o == x).unwrap();
                            let closer_index = closers.iter().position(|&c| c == bracket).unwrap();
                            if opener_index == closer_index {
                                0
                            } else {
                                closer_points[closer_index]
                            }
                        }
                        None => 0,
                    };
                    error_score += score;
                }
            }
            error_score
        })
        .sum()
}

pub fn part_2() -> i64 {
    let openers: [char; 4] = ['(', '{', '<', '['];
    let closers: [char; 4] = [')', '}', '>', ']'];
    let auto_complete_points: [i64; 4] = [1, 3, 4, 2];

    let mut scores: Vec<i64> = fs::read_to_string("inputs/day10")
        .expect("Unable to read input")
        .lines()
        .filter_map(|line| {
            line.chars()
                .fold(Some(Vec::new()), |stack, bracket| match stack {
                    Some(mut stack) => {
                        if openers.contains(&bracket) {
                            stack.push(bracket);
                            Some(stack)
                        } else {
                            match stack.pop() {
                                Some(opener) => {
                                    let opener_index =
                                        openers.iter().position(|&o| o == opener).unwrap();
                                    let closer_index =
                                        closers.iter().position(|&c| c == bracket).unwrap();
                                    if opener_index == closer_index {
                                        Some(stack)
                                    } else {
                                        None
                                    }
                                }
                                None => Some(stack),
                            }
                        }
                    }
                    None => None,
                })
        })
        .map(|stack| {
            stack.iter().rev().fold(0, |score, bracket| {
                let opener_index = openers.iter().position(|&o| o == *bracket).unwrap();
                (score * 5) + auto_complete_points[opener_index]
            })
        })
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}
