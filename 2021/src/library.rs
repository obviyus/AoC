#[macro_export]
macro_rules! scanline {
    ($x:expr) => {
        std::io::stdin().read_line(&mut $x).unwrap()
    };
}

#[macro_export]
macro_rules! content {
    ($path:expr) => {
        std::io::BufReader::new(std::fs::File::open($path).unwrap())
    };
}

#[macro_export]
macro_rules! parse {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! create_benches {
    ({ $($day:ident),+ }) => {
        paste! {
            $(
                use aoc_2021::[<day $day>];

                fn [<bench_day $day _pt1>](c: &mut Criterion) {
                    c.bench_function(&format!("{} pt1", [<day $day>]::TITLE), |b| {
                        b.iter(|| {
                            black_box([<day $day>]::[<day $day _part_1>]());
                        })
                    });
                }
                fn [<bench_day $day _pt2>](c: &mut Criterion) {
                    c.bench_function(&format!("{} pt2", [<day $day>]::TITLE), |b| {
                        b.iter(|| {
                            black_box([<day $day>]::[<day $day _part_2>]());
                        })
                    });
                }
            )+

            criterion_group!(
                benches,
                $(
                    [<bench_day $day _part_1>],
                    [<bench_day $day _part_2>],
                )+
            );
            criterion_main!(benches);
        }
    };
}

#[macro_export]
macro_rules! daychoice {
    ({ $($day:ident),+ }) => {
        paste! {
            $(
                pub mod [<day $day>];
            )+

            #[allow(unused_assignments)]
            fn choice() -> ([fn() -> i64; 2], &'static str) {
                let mut i = 0;
                let mut titles = Vec::new();
            $(
                match i > 9 {
                    true => {
                        println!("{}: {}", i, crate::[<day $day>]::TITLE);
                        titles.push(crate::[<day $day>]::TITLE);
                        i += 1;
                    },
                    false => {
                        println!("{} : {}", i, crate::[<day $day>]::TITLE);
                        titles.push(crate::[<day $day>]::TITLE);
                        i += 1;
                    },
                }
            )+

                print!("Please enter day to run: ");
                let _ = std::io::stdout().flush();

                let mut input = String::new();
                scanline!(input);
                let choice = parse!(input.trim_end(), usize);

                let calls: Vec<[fn() -> i64; 2]> = vec![
            $(
                [ crate::[<day $day>]::[<part_1>], crate::[<day $day>]::[<part_2>] ],
            )+
                ];

                (calls[choice], titles[choice])
            }
        }
    }
}
