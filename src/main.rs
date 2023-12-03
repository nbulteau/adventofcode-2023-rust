use std::collections::HashSet;
use std::env;
use std::io;

use aoc2023::{day01, day02, day03};
use aoc2023::utils::{pretty_print_part_one, pretty_print_part_two};
use aoc2023::utils::get_day_input;

const ADVENT_OF_CODE_DOT_COM: &'static str = "https://adventofcode.com/2023/day/";

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<_>>();

    if args.is_empty() || args.contains("01") {
        let data = get_day_input(1)?;
        println!("--- Day 1: Trebuchet?! ---");
        println!("{}1", ADVENT_OF_CODE_DOT_COM);
        pretty_print_part_one(|| day01::part1(&data));
        pretty_print_part_two(|| day01::part2(&data));
        println!();
    }

    if args.is_empty() || args.contains("02") {
        let data = get_day_input(2)?;
        println!("--- Day 2: Cube Conundrum ---");
        println!("{}2", ADVENT_OF_CODE_DOT_COM);
        pretty_print_part_one(|| day02::part1(&data));
        pretty_print_part_two(|| day02::part2(&data));
        println!();
    }

    if args.is_empty() || args.contains("03") {
        let data = get_day_input(3)?;
        println!("--- Day 3: Gear Ratios ---");
        println!("{}3", ADVENT_OF_CODE_DOT_COM);
        pretty_print_part_one(|| day03::part1(&data));
        pretty_print_part_two(|| day03::part2(&data));
        println!();
    }

    Ok(())
}