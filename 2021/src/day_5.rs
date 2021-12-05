use std::{collections::HashMap, fs};

pub fn part_1() -> i64 {
    fs::read_to_string("inputs/day5")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.split(","))
                .flatten()
                .map(|point| point.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
        .iter()
        .filter(|line| line[0] == line[2] || line[1] == line[2])
        .fold(HashMap::new(), |mut map, line| {
            let start_x = std::cmp::min(line[0], line[2]);
            let end_x = std::cmp::max(line[0], line[2]);

            for x in start_x..=end_x {
                let start_y = std::cmp::min(line[1], line[3]);
                let end_y = std::cmp::max(line[1], line[3]);

                for y in start_y..=end_y {
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }

            map
        })
        .iter()
        .filter_map(|(_, value)| if *value >= 2 { Some(*value) } else { None })
        .count() as i64
}

pub fn part_2() -> i64 {
    fs::read_to_string("inputs/day5")
        .expect("Unable to read input")
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.split(","))
                .flatten()
                .map(|point| point.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
        .iter()
        .fold(HashMap::new(), |mut map, line| {
            let slope_x = match line[0] {
                x if x == line[2] => 0,
                x if x < line[2] => 1,
                x if x > line[2] => -1,
                _ => 0,
            };
            let slope_y = match line[1] {
                y if y == line[3] => 0,
                y if y < line[3] => 1,
                y if y > line[3] => -1,
                _ => 0,
            };

            let mut x = line[0];
            let mut y = line[1];

            *map.entry((x, y)).or_insert(0) += 1;
            while x != line[2] || y != line[3] {
                x += slope_x;
                y += slope_y;

                *map.entry((x, y)).or_insert(0) += 1;
            }

            map
        })
        .iter()
        .filter_map(|(_, value)| if *value >= 2 { Some(*value) } else { None })
        .count() as i64
}
