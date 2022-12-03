use std::{collections::HashSet, fs};

fn common_char_finder(words: Vec<&str>) -> char {
    let mut common_chars = HashSet::<char>::new();
    common_chars.extend(words[0].chars());

    for word in words.iter().skip(1) {
        let word_set = word.chars().collect::<HashSet<char>>();
        common_chars = common_chars
            .intersection(&word_set)
            .map(|c| *c)
            .collect::<HashSet<char>>();
    }

    // Guaranteed to always be a single char
    *common_chars.iter().next().unwrap()
}

fn char_to_priority(char: char) -> i32 {
    if char.is_lowercase() {
        char as i32 - 96
    } else {
        char as i32 - 38
    }
}

pub fn part_1() -> i32 {
    fs::read_to_string("../inputs/day_03.txt")
        .unwrap()
        .split("\n")
        .map(|line| {
            let common_char =
                common_char_finder(vec![&line[0..line.len() / 2], &line[line.len() / 2..]]);
            char_to_priority(common_char)
        })
        .sum()
}

pub fn part_2() -> i32 {
    fs::read_to_string("../inputs/day_03.txt")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let common_char = common_char_finder(vec![&chunk[0], &chunk[1], &chunk[2]]);

            char_to_priority(common_char)
        })
        .sum()
}
