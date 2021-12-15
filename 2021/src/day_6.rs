use std::fs;

pub const TITLE: &str = "--- Day 6: Lanternfish ---";

fn runner(fishes: Vec<i64>, days: i64) -> i64 {
    let mut counter = fishes.clone();

    for _ in 0..days {
        let mut today: Vec<i64> = vec![0; 9];
        for (i, &v) in counter.iter().enumerate() {
            match i {
                0 => {
                    today[6] += v;
                    today[8] += v;
                }
                _ => {
                    today[i - 1] += v;
                }
            }
        }
        counter = today.clone();
    }

    counter.iter().sum()
}

pub fn part_1() -> i64 {
    runner(
        fs::read_to_string("inputs/day6")
            .expect("Unable to read input")
            .lines()
            .map(|line| line.split(','))
            .flatten()
            .map(|n| n.parse::<usize>().unwrap())
            .fold(vec![0 as i64; 9], |mut acc, fish| {
                acc[fish] += 1;
                acc
            }),
        80,
    )
}

pub fn part_2() -> i64 {
    runner(
        fs::read_to_string("inputs/day6")
            .expect("Unable to read input")
            .lines()
            .map(|line| line.split(','))
            .flatten()
            .map(|n| n.parse::<usize>().unwrap())
            .fold(vec![0 as i64; 9], |mut acc, fish| {
                acc[fish] += 1;
                acc
            }),
        256,
    )
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 353079);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 1605400130036);
}
