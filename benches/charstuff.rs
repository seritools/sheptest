use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use sheptest::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let s_ = std::fs::read_to_string("test.xml").unwrap();
    let s = s_
        .chars()
        .filter(|&c| c == '_' || c == ':' || c.is_ascii_alphabetic())
        .collect::<String>();

    c.bench_with_input(BenchmarkId::new("is_name_start_char", "xml"), &s, |b, s| {
        b.iter(|| s.chars().filter(|&c| is_name_start_char(c)).count());
    });
    c.bench_with_input(
        BenchmarkId::new("is_name_start_char_ascii_opt1", "xml"),
        &s,
        |b, s| {
            b.iter(|| {
                s.chars()
                    .filter(|&c| is_name_start_char_ascii_opt1(c))
                    .count()
            });
        },
    );
    c.bench_with_input(
        BenchmarkId::new("is_name_start_char_ascii_opt2", "xml"),
        &s,
        |b, s| {
            b.iter(|| {
                s.chars()
                    .filter(|&c| is_name_start_char_ascii_opt2(c))
                    .count()
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
