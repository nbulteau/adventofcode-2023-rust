use std::io;

use criterion::{black_box, Criterion};

use aoc2023::{day01, day02, day03};
use aoc2023::utils::get_day_input;

/// Get the input for a given day.
fn aoc2023_bench(criterion: &mut Criterion) -> Result<(), io::Error> {
    let data = get_day_input(1)?;
    let mut group = criterion.benchmark_group("Day 1");
    group.bench_function("Part one answer : ", |b| b.iter(|| day01::part1(black_box(&data))));
    group.bench_function("Part two answer : ", |b| b.iter(|| day01::part2(black_box(&data))));
    group.finish();

    let data = get_day_input(2)?;
    let mut group = criterion.benchmark_group("Day 2");
    group.bench_function("Part one answer : ", |b| b.iter(|| day02::part1(black_box(&data))));
    group.bench_function("Part two answer : ", |b| b.iter(|| day02::part2(black_box(&data))));
    group.finish();

    let data = get_day_input(3)?;
    let mut group = criterion.benchmark_group("Day 3");
    group.bench_function("Part one answer : ", |b| b.iter(|| day03::part1(black_box(&data))));
    group.bench_function("Part two answer : ", |b| b.iter(|| day03::part2(black_box(&data))));
    group.finish();

    Ok(())
}

fn aoc2023() -> Result<(), io::Error> {
    let mut criterion = Criterion::default().configure_from_args();
    aoc2023_bench(&mut criterion)?;
    Ok(())
}

fn main() -> Result<(), io::Error> {
    aoc2023()?;
    Criterion::default().configure_from_args().final_summary();
    Ok(())
}