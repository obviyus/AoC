use std::{collections::HashSet, fs};

pub const TITLE: &str = "--- Day 11: Dumbo Octopus ---";

fn neighbors(grid: Vec<Vec<i64>>, x: i64, y: i64) -> Vec<(usize, usize)> {
    // Return the indices of the neighbors of the given cell including diagonals
    let mut neighbors = vec![];

    for (p, q) in vec![
        (x + 1, y),
        (x - 1, y),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x, y + 1),
        (x, y - 1),
    ] {
        if p >= 0 && q >= 0 && p < grid[0].len() as i64 && q < grid[0].len() as i64 {
            neighbors.push((p as usize, q as usize));
        }
    }

    neighbors
}

pub fn part_1() -> i64 {
    let mut grid: Vec<Vec<i64>> = fs::read_to_string("inputs/day11")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    (0..100)
        .into_iter()
        .map(|_| {
            let mut queue: Vec<(usize, usize)> = grid
                .iter_mut()
                .enumerate()
                .filter_map(|(x, row)| {
                    Some(row.iter_mut().enumerate().filter_map(move |(y, cell)| {
                        *cell += 1;
                        if *cell > 9 {
                            Some((x, y))
                        } else {
                            None
                        }
                    }))
                })
                .flatten()
                .collect();

            let mut flashed: HashSet<(usize, usize)> = HashSet::from_iter(queue.clone());
            while let Some((x, y)) = queue.pop() {
                grid[x][y] = 0;

                for (p, q) in neighbors(grid.clone(), x as i64, y as i64) {
                    if !flashed.contains(&(p, q)) {
                        grid[p][q] += 1;
                        if grid[p][q] > 9 {
                            flashed.insert((p, q));
                            queue.push((p, q));
                        }
                    }
                }
            }

            flashed.len() as i64
        })
        .sum()
}

pub fn part_2() -> i64 {
    let mut grid: Vec<Vec<i64>> = fs::read_to_string("inputs/day11")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut steps = 1;
    loop {
        let mut queue: Vec<(usize, usize)> = grid
            .iter_mut()
            .enumerate()
            .filter_map(|(x, row)| {
                Some(row.iter_mut().enumerate().filter_map(move |(y, cell)| {
                    *cell += 1;
                    if *cell > 9 {
                        Some((x, y))
                    } else {
                        None
                    }
                }))
            })
            .flatten()
            .collect();

        let mut flashed: HashSet<(usize, usize)> = HashSet::from_iter(queue.clone());
        while let Some((x, y)) = queue.pop() {
            grid[x][y] = 0;

            for (p, q) in neighbors(grid.clone(), x as i64, y as i64) {
                if !flashed.contains(&(p, q)) {
                    grid[p][q] += 1;
                    if grid[p][q] > 9 {
                        flashed.insert((p, q));
                        queue.push((p, q));
                    }
                }
            }
        }
        if flashed.len() == 100 {
            return steps;
        }
        steps += 1;
    }
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 1721);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 298);
}
