use std::{collections::HashMap, fs};

pub const TITLE: &str = "--- Day 14: Extended Polymerization ---";

fn steps(steps: i64, template: String, rules: HashMap<String, String>) -> i64 {
    let mut pairs: HashMap<String, i64> = template.chars().zip(template.chars().skip(1)).fold(
        HashMap::new(),
        |mut counter, (x, y)| {
            *counter.entry(x.to_string() + &y.to_string()).or_insert(0) += 1;
            counter
        },
    );

    let mut chars: HashMap<String, i64> =
        template.chars().fold(HashMap::new(), |mut counter, x| {
            *counter.entry(x.to_string()).or_insert(0) += 1;
            counter
        });

    for _ in 0..steps {
        for (pair, count) in &pairs.clone() {
            match rules.get(pair) {
                Some(to_insert) => {
                    let split = pair
                        .chars()
                        .map(|char| char.to_string())
                        .collect::<Vec<String>>();

                    *pairs.get_mut(pair).unwrap() -= count;
                    *pairs.entry(split[0].clone() + to_insert).or_insert(0) += count;
                    *pairs.entry(to_insert.clone() + &split[1]).or_insert(0) += count;

                    *chars.entry(to_insert.clone()).or_insert(0) += count;
                }
                None => (),
            }
        }
    }

    chars.values().max().unwrap() - chars.values().min().unwrap()
}

pub fn part_1() -> i64 {
    let input: Vec<String> = fs::read_to_string("inputs/day14")
        .expect("Unable to read input")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let rules: HashMap<String, String> = input.iter().filter(|line| line.contains(" -> ")).fold(
        HashMap::new(),
        |mut rules, line| {
            let mut split = line.split(" -> ");
            let parent = split.next().unwrap();
            let child = split.next().unwrap();

            rules.insert(parent.to_string(), child.to_string());
            rules
        },
    );

    steps(10, input[0].clone(), rules)
}

pub fn part_2() -> i64 {
    let input: Vec<String> = fs::read_to_string("inputs/day14")
        .expect("Unable to read input")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let rules: HashMap<String, String> = input.iter().filter(|line| line.contains(" -> ")).fold(
        HashMap::new(),
        |mut rules, line| {
            let mut split = line.split(" -> ");
            let parent = split.next().unwrap();
            let child = split.next().unwrap();

            rules.insert(parent.to_string(), child.to_string());
            rules
        },
    );

    steps(40, input[0].clone(), rules)
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 3555);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 4439442043739);
}
