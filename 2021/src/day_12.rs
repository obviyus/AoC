use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub const TITLE: &str = "--- Day 12: Passage Pathing ---";

fn traverse(
    current_node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: HashSet<&str>,
    mut part: u8,
) -> i64 {
    if current_node == "end" {
        return 1;
    }

    if visited.contains(current_node) {
        if current_node == "start" {
            return 0;
        }

        if current_node.chars().next().unwrap().is_lowercase() {
            if part == 1 {
                return 0;
            } else {
                part = 1;
            }
        }
    }

    let mut new_visited = visited.clone();
    new_visited.insert(current_node);

    graph
        .get(current_node)
        .unwrap()
        .iter()
        .map(|next_node| traverse(next_node, graph, new_visited.clone(), part))
        .sum()
}

pub fn part_1() -> i64 {
    let graph: HashMap<String, Vec<String>> = fs::read_to_string("inputs/day12")
        .expect("Unable to read input")
        .lines()
        .fold(HashMap::new(), |mut graph, line| {
            let mut split = line.split("-");
            let parent = split.next().unwrap();
            let child = split.next().unwrap();
            graph
                .entry(parent.to_string())
                .or_insert(vec![])
                .push(child.to_string());
            graph
                .entry(child.to_string())
                .or_insert(vec![])
                .push(parent.to_string());

            graph
        });

    traverse("start", &graph, HashSet::new(), 1)
}

pub fn part_2() -> i64 {
    let graph: HashMap<String, Vec<String>> = fs::read_to_string("inputs/day12")
        .expect("Unable to read input")
        .lines()
        .fold(HashMap::new(), |mut graph, line| {
            let mut split = line.split("-");
            let parent = split.next().unwrap();
            let child = split.next().unwrap();
            graph
                .entry(parent.to_string())
                .or_insert(vec![])
                .push(child.to_string());
            graph
                .entry(child.to_string())
                .or_insert(vec![])
                .push(parent.to_string());

            graph
        });

    traverse("start", &graph, HashSet::new(), 2)
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 4573);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 117509);
}
