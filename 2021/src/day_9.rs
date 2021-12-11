use std::{
    collections::{BinaryHeap, HashSet},
    fs,
};

pub const TITLE: &str = "--- Day 9: Smoke Basin ---";

fn is_low_point(grid: &Vec<Vec<i64>>, x: i64, y: i64) -> bool {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    for &(p, q) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
        if p >= 0
            && p < height
            && q >= 0
            && q < width
            && grid[p as usize][q as usize] <= grid[x as usize][y as usize]
        {
            return false;
        }
    }
    true
}

fn neighbors(grid: &Vec<Vec<i64>>, x: i64, y: i64) -> Vec<(usize, usize)> {
    let height = grid.len() as i64;
    let width = grid[0].len() as i64;
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for &(p, q) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
        if p >= 0 && p < height && q >= 0 && q < width && grid[p as usize][q as usize] != 9 {
            neighbors.push((p as usize, q as usize));
        }
    }

    neighbors
}

pub fn part_1() -> i64 {
    let grid: Vec<Vec<i64>> = fs::read_to_string("inputs/day9")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut total = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, &val) in row.iter().enumerate() {
            if is_low_point(&grid, x as i64, y as i64) {
                total += val + 1;
            }
        }
    }

    total
}

pub fn part_2() -> i64 {
    let grid: Vec<Vec<i64>> = fs::read_to_string("inputs/day9")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let low_points = grid
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, _)| is_low_point(&grid, x as i64, *y as i64))
                .map(|(y, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize)>>();

    let mut basins: BinaryHeap<usize> = BinaryHeap::new();
    for &(x, y) in low_points.iter() {
        let mut queue = vec![(x, y)];
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        while let Some((x, y)) = queue.pop() {
            if !seen.contains(&(x, y)) {
                seen.insert((x, y));
                neighbors(&grid, x as i64, y as i64)
                    .iter()
                    .for_each(|(p, q)| {
                        queue.push((*p, *q));
                    });
            }
        }
        basins.push(seen.len());
    }

    (basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()) as i64
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 562);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 1076922);
}
