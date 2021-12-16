use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc_2021::{
    day_1, day_10, day_11, day_12, day_13, day_14, day_15, day_2, day_3, day_4, day_5, day_6,
    day_7, day_8, day_9,
};

fn bench_day_1_part_1(c: &mut Criterion) {
    c.bench_function("day_1_part_1", |b| {
        b.iter(|| {
            black_box(day_1::part_1());
        })
    });
}

fn bench_day_1_part_2(c: &mut Criterion) {
    c.bench_function("day_1_part_2", |b| {
        b.iter(|| {
            black_box(day_1::part_2());
        })
    });
}

fn bench_day_2_part_1(c: &mut Criterion) {
    c.bench_function("day_2_part_1", |b| {
        b.iter(|| {
            black_box(day_2::part_1());
        })
    });
}

fn bench_day_2_part_2(c: &mut Criterion) {
    c.bench_function("day_2_part_2", |b| {
        b.iter(|| {
            black_box(day_2::part_2());
        })
    });
}

fn bench_day_3_part_1(c: &mut Criterion) {
    c.bench_function("day_3_part_1", |b| {
        b.iter(|| {
            black_box(day_3::part_1());
        })
    });
}

fn bench_day_3_part_2(c: &mut Criterion) {
    c.bench_function("day_3_part_2", |b| {
        b.iter(|| {
            black_box(day_3::part_2());
        })
    });
}

fn bench_day_4_part_1(c: &mut Criterion) {
    c.bench_function("day_4_part_1", |b| {
        b.iter(|| {
            black_box(day_4::part_1());
        })
    });
}

fn bench_day_4_part_2(c: &mut Criterion) {
    c.bench_function("day_4_part_2", |b| {
        b.iter(|| {
            black_box(day_4::part_2());
        })
    });
}

fn bench_day_5_part_1(c: &mut Criterion) {
    c.bench_function("day_5_part_1", |b| {
        b.iter(|| {
            black_box(day_5::part_1());
        })
    });
}

fn bench_day_5_part_2(c: &mut Criterion) {
    c.bench_function("day_5_part_2", |b| {
        b.iter(|| {
            black_box(day_5::part_2());
        })
    });
}

fn bench_day_6_part_1(c: &mut Criterion) {
    c.bench_function("day_6_part_1", |b| {
        b.iter(|| {
            black_box(day_6::part_1());
        })
    });
}

fn bench_day_6_part_2(c: &mut Criterion) {
    c.bench_function("day_6_part_2", |b| {
        b.iter(|| {
            black_box(day_6::part_2());
        })
    });
}

fn bench_day_7_part_1(c: &mut Criterion) {
    c.bench_function("day_7_part_1", |b| {
        b.iter(|| {
            black_box(day_7::part_1());
        })
    });
}

fn bench_day_7_part_2(c: &mut Criterion) {
    c.bench_function("day_7_part_2", |b| {
        b.iter(|| {
            black_box(day_7::part_2());
        })
    });
}

fn bench_day_8_part_1(c: &mut Criterion) {
    c.bench_function("day_8_part_1", |b| {
        b.iter(|| {
            black_box(day_8::part_1());
        })
    });
}

fn bench_day_8_part_2(c: &mut Criterion) {
    c.bench_function("day_8_part_2", |b| {
        b.iter(|| {
            black_box(day_8::part_2());
        })
    });
}

fn bench_day_9_part_1(c: &mut Criterion) {
    c.bench_function("day_9_part_1", |b| {
        b.iter(|| {
            black_box(day_9::part_1());
        })
    });
}

fn bench_day_9_part_2(c: &mut Criterion) {
    c.bench_function("day_9_part_2", |b| {
        b.iter(|| {
            black_box(day_9::part_2());
        })
    });
}

fn bench_day_10_part_1(c: &mut Criterion) {
    c.bench_function("day_10_part_1", |b| {
        b.iter(|| {
            black_box(day_10::part_1());
        })
    });
}

fn bench_day_10_part_2(c: &mut Criterion) {
    c.bench_function("day_10_part_2", |b| {
        b.iter(|| {
            black_box(day_10::part_2());
        })
    });
}

fn bench_day_11_part_1(c: &mut Criterion) {
    c.bench_function("day_11_part_1", |b| {
        b.iter(|| {
            black_box(day_11::part_1());
        })
    });
}

fn bench_day_11_part_2(c: &mut Criterion) {
    c.bench_function("day_11_part_2", |b| {
        b.iter(|| {
            black_box(day_11::part_2());
        })
    });
}

fn bench_day_12_part_1(c: &mut Criterion) {
    c.bench_function("day_12_part_1", |b| {
        b.iter(|| {
            black_box(day_12::part_1());
        })
    });
}

fn bench_day_12_part_2(c: &mut Criterion) {
    c.bench_function("day_12_part_2", |b| {
        b.iter(|| {
            black_box(day_12::part_2());
        })
    });
}

fn bench_day_13_part_1(c: &mut Criterion) {
    c.bench_function("day_13_part_1", |b| {
        b.iter(|| {
            black_box(day_13::part_1());
        })
    });
}

fn bench_day_13_part_2(c: &mut Criterion) {
    c.bench_function("day_13_part_2", |b| {
        b.iter(|| {
            black_box(day_13::part_2());
        })
    });
}

fn bench_day_14_part_1(c: &mut Criterion) {
    c.bench_function("day_14_part_1", |b| {
        b.iter(|| {
            black_box(day_14::part_1());
        })
    });
}

fn bench_day_14_part_2(c: &mut Criterion) {
    c.bench_function("day_14_part_2", |b| {
        b.iter(|| {
            black_box(day_14::part_2());
        })
    });
}

fn bench_day_15_part_1(c: &mut Criterion) {
    c.bench_function("day_15_part_1", |b| {
        b.iter(|| {
            black_box(day_15::part_1());
        })
    });
}

fn bench_day_15_part_2(c: &mut Criterion) {
    c.bench_function("day_15_part_2", |b| {
        b.iter(|| {
            black_box(day_15::part_2());
        })
    });
}

criterion_group!(
    benches,
    bench_day_1_part_1,
    bench_day_1_part_2,
    bench_day_2_part_1,
    bench_day_2_part_2,
    bench_day_3_part_1,
    bench_day_3_part_2,
    bench_day_4_part_1,
    bench_day_4_part_2,
    bench_day_5_part_1,
    bench_day_5_part_2,
    bench_day_6_part_1,
    bench_day_6_part_2,
    bench_day_7_part_1,
    bench_day_7_part_2,
    bench_day_8_part_1,
    bench_day_8_part_2,
    bench_day_9_part_1,
    bench_day_9_part_2,
    bench_day_10_part_1,
    bench_day_10_part_2,
    bench_day_11_part_1,
    bench_day_11_part_2,
    bench_day_12_part_1,
    bench_day_12_part_2,
    bench_day_13_part_1,
    bench_day_13_part_2,
    bench_day_14_part_1,
    bench_day_14_part_2,
    bench_day_15_part_1,
    bench_day_15_part_2
);
criterion_main!(benches);
