pub const TITLE: &str = "--- Day 13: Transparent Origami ---";

use std::{collections::HashSet, fs};

fn folder(axis: char, position: i64, dots: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut update: HashSet<(i64, i64)> = HashSet::with_capacity(dots.len());
    match axis {
        'x' => {
            for (x, y) in dots.iter() {
                if *x > position {
                    update.insert((2 * position - *x, *y));
                } else {
                    update.insert((*x, *y));
                }
            }
        }
        'y' => {
            for (x, y) in dots.iter() {
                if *y > position {
                    update.insert((*x, 2 * position - *y));
                } else {
                    update.insert((*x, *y));
                }
            }
        }
        _ => panic!("Invalid axis!"),
    }

    update
}

pub fn part_1() -> i64 {
    let mut once = false;

    fs::read_to_string("inputs/day13")
        .expect("Unable to read input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .fold(HashSet::new(), |mut dots, line| {
            match line.starts_with("fold along") {
                false => {
                    let mut point = line.split(",");
                    let x = point.next().unwrap().parse::<i64>().unwrap();
                    let y = point.next().unwrap().parse::<i64>().unwrap();
                    dots.insert((x, y));

                    dots
                }
                true => match once {
                    false => {
                        once = true;
                        let mut point = line.split("=");
                        let axis = point.next().unwrap().chars().last().unwrap();
                        let position = point.next().unwrap().parse::<i64>().unwrap();

                        folder(axis, position, dots)
                    }
                    true => dots,
                },
            }
        })
        .len() as i64
}

pub fn part_2() -> i64 {
    fs::read_to_string("inputs/day13")
        .expect("Unable to read input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .fold(HashSet::new(), |mut dots, line| {
            match line.starts_with("fold along") {
                false => {
                    let mut point = line.split(",");
                    let x = point.next().unwrap().parse::<i64>().unwrap();
                    let y = point.next().unwrap().parse::<i64>().unwrap();
                    dots.insert((x, y));

                    dots
                }
                true => {
                    let mut point = line.split("=");
                    let axis = point.next().unwrap().chars().last().unwrap();
                    let position = point.next().unwrap().parse::<i64>().unwrap();

                    folder(axis, position, dots)
                }
            }
        })
        .len() as i64
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 775);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 102);
}
