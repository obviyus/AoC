use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

mod library;
create_benches!({ _0, _1, _2, _3, _4,
   _5, _6, _7, _8, _9,
   _10, _11, _12, _13, _14,
   _15
});
