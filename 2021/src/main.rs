use paste::paste;
use std::io::Write;
use std::time::Instant;

#[macro_use]
mod library;

#[allow(unused_must_use)]
fn main() {
    let mut input = String::new();
    // Thank you JoseAlPaca for the "framework" :)

    print!("Do you wish to run (o)nce or (b)enchmark your solution with x iterations: ");
    std::io::stdout().flush();

    scanline!(input);
    let input = input.trim_end();
    let choice = choice();

    if input.to_ascii_lowercase() == "o" {
        for (i, f) in choice.0.iter().enumerate() {
            println!("Part #{}: {}", i + 1, f());
        }
    } else {
        let i;
        if input.to_ascii_lowercase() == "b" {
            i = 10000;
        } else {
            i = input.parse::<i32>().unwrap();
        }

        println!("\nBenchmarking \"{}\". Please wait.", choice.1);
        for (n, f) in choice.0.iter().enumerate() {
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
            println!("Part #{}: {}", n + 1, f());
        }
    }
}

daychoice!({ _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11});
