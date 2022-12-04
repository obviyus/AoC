use std::fs;

pub fn part_1() -> i32 {
    fs::read_to_string("../inputs/day_04.txt")
        .unwrap()
        .split("\n")
        .filter(|line| {
            let parsed_ranges = line
                .split(",")
                .map(|range| {
                    let range = range.split("-").collect::<Vec<&str>>();
                    let start = range[0].parse::<i32>().unwrap();
                    let end = range[1].parse::<i32>().unwrap();

                    (start, end)
                })
                .collect::<Vec<(i32, i32)>>();

            parsed_ranges[0].0 >= parsed_ranges[1].0 && parsed_ranges[0].1 <= parsed_ranges[1].1
                || parsed_ranges[1].0 >= parsed_ranges[0].0
                    && parsed_ranges[1].1 <= parsed_ranges[0].1
        })
        .count() as i32
}

pub fn part_2() -> i32 {
    fs::read_to_string("../inputs/day_04.txt")
        .unwrap()
        .split("\n")
        .filter(|line| {
            let parsed_ranges = line
                .split(",")
                .map(|range| {
                    let range = range.split("-").collect::<Vec<&str>>();
                    let start = range[0].parse::<i32>().unwrap();
                    let end = range[1].parse::<i32>().unwrap();

                    (start, end)
                })
                .collect::<Vec<(i32, i32)>>();

            parsed_ranges[0].0 <= parsed_ranges[1].0 && parsed_ranges[0].1 >= parsed_ranges[1].0
                || parsed_ranges[1].0 <= parsed_ranges[0].0
                    && parsed_ranges[1].1 >= parsed_ranges[0].0
        })
        .count() as i32
}
