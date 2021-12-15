pub const TITLE: &str = "--- Day 4: Giant Squid ---";

use std::{collections::BinaryHeap, fs};

fn is_present(board: &Vec<Vec<i64>>, number: i64) -> Option<(usize, usize)> {
    // Return x, y if number is present in board
    board.iter().enumerate().find_map(|(x, row)| {
        row.iter()
            .enumerate()
            .find_map(|(y, &n)| if n == number { Some((x, y)) } else { None })
    })
}

fn sum_of_unmarked(board: &Vec<Vec<i64>>, marked: &Vec<Vec<bool>>) -> i64 {
    // Return sum of all numbers in board that are marked as false
    board.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(
            acc,
            |acc, (x, &n)| {
                if !marked[y][x] {
                    acc + n
                } else {
                    acc
                }
            },
        )
    })
}

fn is_solved(marked: &Vec<Vec<bool>>) -> bool {
    // Check if at least one row or column in board is all marked
    marked.iter().any(|row| row.iter().all(|&n| n))
        || (0..marked[0].len()).any(|x| marked.iter().map(|row| row[x]).all(|n| n))
}

pub fn part_1() -> i64 {
    let input = fs::read_to_string("inputs/day4")
        .expect("Unable to read input")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let bingo_numbers = input[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut results: BinaryHeap<(i64, i64, i64)> = BinaryHeap::with_capacity(bingo_numbers.len());
    let mut current_board: Vec<Vec<i64>> = Vec::new();

    for line in input.iter().skip(1) {
        if line.is_empty() {
            let mut time_to_solve = 0;
            let mut current_marked = vec![vec![false; 5]; 5];

            for current_number in bingo_numbers.iter() {
                if let Some((x, y)) = is_present(&current_board, *current_number) {
                    current_marked[x][y] = true;
                    if is_solved(&current_marked) {
                        results.push((
                            time_to_solve * -1,
                            *current_number,
                            sum_of_unmarked(&current_board, &current_marked),
                        ));
                        break;
                    }
                }

                time_to_solve += 1;
            }

            current_board.clear();
        } else {
            current_board.push(
                line.split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            );
        }
    }

    let ans = results.pop().unwrap();
    ans.1 * ans.2
}

pub fn part_2() -> i64 {
    0
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 6592);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 31755);
}
