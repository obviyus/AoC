mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;

#[macro_use]
extern crate lazy_static;
fn main() {
    println!("{}", format!("Day 1 Part 1: {}", day_01::part_1()));
    println!("{}", format!("Day 1 Part 2: {}", day_01::part_2()));

    println!("{}", format!("Day 2 Part 1: {}", day_02::part_1()));
    println!("{}", format!("Day 2 Part 2: {}", day_02::part_2()));

    println!("{}", format!("Day 3 Part 1: {}", day_03::part_1()));
    println!("{}", format!("Day 3 Part 2: {}", day_03::part_2()));

    println!("{}", format!("Day 4 Part 1: {}", day_04::part_1()));
    println!("{}", format!("Day 4 Part 2: {}", day_04::part_2()));

    println!("{}", format!("Day 6 Part 1: {}", day_06::part_1()));
    println!("{}", format!("Day 6 Part 2: {}", day_06::part_2()));
}
