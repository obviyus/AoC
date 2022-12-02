mod day_01;
mod day_02;

#[macro_use]
extern crate lazy_static;
fn main() {
    println!("{}", format!("Day 1 Part 1: {}", day_01::part_1()));
    println!("{}", format!("Day 1 Part 2: {}", day_01::part_2()));

    println!("{}", format!("Day 2 Part 1: {}", day_02::part_1()));
    println!("{}", format!("Day 2 Part 2: {}", day_02::part_2()));
}
