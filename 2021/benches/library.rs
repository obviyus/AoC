#[allow(unused_macros)]
#[macro_export]
macro_rules! create_benches {
    ({ $($day:ident),+ }) => {
        paste! {
            $(
                use aoc_2021::[<day $day>];

                fn [<bench_day $day _part_1>](c: &mut Criterion) {
                    c.bench_function(&format!("{} part_1", [<day $day>]::TITLE), |b| {
                        b.iter(|| {
                            black_box([<day $day>]::[<part_1>]());
                        })
                    });
                }
                fn [<bench_day $day _part_2>](c: &mut Criterion) {
                    c.bench_function(&format!("{} part_2", [<day $day>]::TITLE), |b| {
                        b.iter(|| {
                            black_box([<day $day>]::[<part_2>]());
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
