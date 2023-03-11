use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

extern crate sudoku_solver;
use sudoku_solver::basic::basic_solve;
use sudoku_solver::bitfield::bitfield_solve;
use sudoku_solver::commons::solutions;

fn bitfield_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitfield");
    let duration = Duration::new(0, 1_000_000_000);
    group
        .significance_level(0.01)
        .sample_size(100)
        .warm_up_time(duration)
        .measurement_time(duration);

    group.bench_function("hard", |b| {
        b.iter(|| {
            bitfield_solve(
                "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......",
            )
        })
    });
    group.bench_function("easy", |b| {
        b.iter(|| {
            bitfield_solve(
                "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....",
            )
        })
    });
    group.bench_function("10k", |b| {
        b.iter(|| solutions("hard_sudokus.txt", bitfield_solve))
    });
    group.bench_function("50k", |b| {
        b.iter(|| solutions("all_17_clue_sudokus.txt", bitfield_solve))
    });

    group.finish();
}

fn basic_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic");
    let duration = Duration::new(0, 1_000_000_000);
    group
        .significance_level(0.01)
        .sample_size(10)
        .warm_up_time(duration)
        .measurement_time(duration);

    group.bench_function("hard", |b| {
        b.iter(|| {
            basic_solve(
                "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......",
            )
        })
    });
    group.bench_function("easy", |b| {
        b.iter(|| {
            basic_solve(
                "....79.65.....3..2..5.6..9334..5.1.6.........6.8.2..5995..1.6..7..6.....82.39....",
            )
        })
    });
    group.bench_function("10k", |b| {
        b.iter(|| solutions("hard_sudokus.txt", basic_solve))
    });
    // // TOO SLOW
    // group.bench_function("50k", |b| {
    //     b.iter(|| solutions("all_17_clue_sudokus.txt", basic_solve))
    // });

    group.finish();
}

criterion_group!(benches, bitfield_bench, basic_bench);
criterion_main!(benches);
