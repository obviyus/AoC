use std::io::Write;
use std::time::Instant;

// Thanks JoseAlPaca for the Rust "framework" :D

#[macro_use]
mod library;
mod day_1;
/*mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6_2;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;*/

#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    //let choice;

    print!("Do you wish to run (o)nce or benchmark your solution with (x) iterations: ");
    std::io::stdout().flush();

    scanline!(input);
    let input = input.trim_end();
    let choice = choice();

    if input.to_ascii_lowercase() == "o" {
        for (i, f) in choice.iter().enumerate() {
            println!("Answer #{}: {}", i + 1, f());
        }
    } else {
        let i = input.parse::<i32>().unwrap();

        for (n, f) in choice.iter().enumerate() {
            let now = Instant::now();
            for _ in 1..i {
                f();
            }
            let elapsed = now.elapsed().as_nanos();
            println!(
                "\nExercise ran {} times in {}ns ({:.03}s).",
                i,
                elapsed,
                elapsed as f64 / 1000000000_f64
            );
            println!("Answer #{}: {}", n + 1, f());
        }
    }
}

#[allow(unused_must_use)]
fn choice() -> [fn() -> i64; 2] {
    print!("Please enter day to run: ");
    std::io::stdout().flush();

    let mut input = String::new();
    scanline!(input);
    let choice = parse!(input.trim_end(), u8);

    let r1;
    let r2;

    match choice {
        x if x == 1 => {
            r1 = crate::day_1::part_1;
            r2 = crate::day_1::part_2;
        }
        x => panic!("Day {} not found.", x),
    }

    [r1, r2]
}
