use criterion::{criterion_group, criterion_main, Criterion};
use sprite_gen::*;

fn criterion_benchmark(c: &mut Criterion) {
    let buffer_10x10: Vec<i8> = (0..10 * 10).map(|index| index % 3 - 1).collect();
    let buffer_100x100: Vec<i8> = (0..100 * 100).map(|index| (index % 3 - 1) as i8).collect();

    c.bench_function("gen color 10x10", |b| {
        let mut seed = 0;
        b.iter(|| {
            let result = gen_sprite(
                &buffer_10x10,
                10,
                Options {
                    colored: true,
                    seed,
                    ..Default::default()
                },
            );
            assert_eq!(result.len(), 100);
            seed += 1;
        });
    });
    c.bench_function("gen color 100x100", |b| {
        let mut seed = 0;
        b.iter(|| {
            let result = gen_sprite(
                &buffer_100x100,
                100,
                Options {
                    colored: true,
                    seed,
                    ..Default::default()
                },
            );
            assert_eq!(result.len(), 100 * 100);
            seed += 1;
        });
    });
    c.bench_function("gen bw 10x10", |b| {
        let mut seed = 0;
        b.iter(|| {
            let result = gen_sprite(
                &buffer_10x10,
                10,
                Options {
                    colored: false,
                    seed,
                    ..Default::default()
                },
            );
            assert_eq!(result.len(), 100);
            seed += 1;
        });
    });
    c.bench_function("gen bw 100x100", |b| {
        let mut seed = 0;
        b.iter(|| {
            let result = gen_sprite(
                &buffer_100x100,
                100,
                Options {
                    colored: false,
                    seed,
                    ..Default::default()
                },
            );
            assert_eq!(result.len(), 100 * 100);
            seed += 1;
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
