use std::{collections::HashMap, fs};

lazy_static! {
    static ref MY_MOVE_MAP: HashMap<&'static str, u8> =
        HashMap::from([("X", 0), ("Y", 1), ("Z", 2),]);
    static ref OPPONENT_MOVE_MAP: HashMap<&'static str, u8> =
        HashMap::from([("A", 0), ("B", 1), ("C", 2),]);
}

pub fn compute_score(my_move: i8, opponent_move: i8) -> i8 {
    let winning_move = (opponent_move + 1) % 3;
    let score = my_move + 1;

    if my_move == opponent_move {
        score + 3
    } else if my_move == winning_move {
        score + 6
    } else {
        score
    }
}

pub fn part_1() -> i32 {
    fs::read_to_string("../inputs/day_02.txt")
        .unwrap()
        .split("\n")
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();

            let opponent_move = OPPONENT_MOVE_MAP.get(moves[0]).unwrap();
            let my_move = MY_MOVE_MAP.get(moves[1]).unwrap();

            compute_score(*my_move as i8, *opponent_move as i8) as i32
        })
        .sum()
}

pub fn part_2() -> i32 {
    fs::read_to_string("../inputs/day_02.txt")
        .unwrap()
        .split("\n")
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();

            let opponent_move = OPPONENT_MOVE_MAP.get(moves[0]).unwrap();
            let game_result = moves[1];

            let mut my_move = 0;
            if game_result == "X" {
                my_move = (opponent_move + 2) % 3;
            } else if game_result == "Y" {
                my_move = *opponent_move;
            } else if game_result == "Z" {
                my_move = (opponent_move + 1) % 3;
            }

            compute_score(my_move as i8, *opponent_move as i8) as i32
        })
        .sum()
}
