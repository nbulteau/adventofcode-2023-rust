use std::io;
use std::io::Error;

use criterion::{black_box, Criterion};

use aoc2023::{day01, day02, day03};
use aoc2023::utils::get_day_input;

/// Get the input for a given day.
fn aoc2023_bench(criterion: &mut Criterion) -> Result<(), io::Error> {
    launch_bench(criterion, 1, day01::part_one, day01::part_two)?;
    launch_bench(criterion, 2, day02::part_one, day02::part_two)?;
    launch_bench(criterion, 3, day03::part_one, day03::part_two)?;
    launch_bench(criterion, 4, day03::part_one, day03::part_two)?;

    Ok(())
}

fn launch_bench(criterion: &mut Criterion, day: u8, part1: fn(&str) -> u32, part2: fn(&str) -> u32) -> Result<(), Error> {
    let data = get_day_input(day)?;
    let mut group = criterion.benchmark_group(format!("Day {}", day));
    group.bench_function("Part one answer : ", |b| b.iter(|| part1(black_box(&data))));
    group.bench_function("Part two answer : ", |b| b.iter(|| part2(black_box(&data))));
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