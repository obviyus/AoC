pub const TITLE: &str = "--- Day 15: Chiton ---";

use std::{collections::BinaryHeap, fs};

fn neighbors(grid: &Vec<Vec<i64>>, x: i64, y: i64) -> Vec<(usize, usize)> {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for &(p, q) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
        if p >= 0 && p < height && q >= 0 && q < width {
            neighbors.push((p as usize, q as usize));
        }
    }

    neighbors
}

fn maze_solver(grid: &Vec<Vec<i64>>) -> i64 {
    let mut costs: Vec<Vec<i64>> = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    let mut queue: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();
    queue.push((0, 0, 0));

    while let Some((cost, x, y)) = queue.pop() {
        if x == grid.len() - 1 && y == grid[0].len() - 1 {
            return -1 * cost;
        }

        if costs[x][y] < -1 * cost {
            continue;
        }

        for &(p, q) in neighbors(&grid, x as i64, y as i64).iter() {
            let new_cost = -1 * cost + grid[p][q];
            if new_cost < costs[p][q] {
                costs[p][q] = new_cost;
                queue.push((-1 * new_cost, p, q));
            }
        }
    }
    0
}

pub fn part_1() -> i64 {
    maze_solver(
        &fs::read_to_string("inputs/day15")
            .expect("Unable to read input")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| digit.to_digit(10).unwrap() as i64)
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>(),
    )
}

pub fn part_2() -> i64 {
    let grid = fs::read_to_string("inputs/day15")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    maze_solver(
        &(0..(5 * grid.len()))
            .map(|x| {
                (0..(5 * grid[0].len()))
                    .map(|y| {
                        let cost = grid[x % grid.len()][y % grid[0].len()]
                            + (x / grid.len()) as i64
                            + (y / grid[0].len()) as i64;
                        if cost < 10 {
                            cost
                        } else {
                            cost - 9
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 458);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 2800);
}
