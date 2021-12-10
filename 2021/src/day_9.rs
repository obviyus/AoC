use std::fs;

pub const TITLE: &str = "--- Day 9: Smoke Basin ---";

fn is_low_point(grid: &Vec<Vec<i64>>, x: i64, y: i64) -> bool {
    let HEIGHT = grid.len() as i64;
    let WIDTH = grid[0].len() as i64;
    for &(p, q) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
        if p >= 0
            && p < HEIGHT
            && q >= 0
            && q < WIDTH
            && grid[p as usize][q as usize] >= grid[x as usize][y as usize]
        {
            return false;
        }
    }
    true
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
    // do nothing faster
    0
}
