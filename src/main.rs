use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Error;

use aoc2023::{day01, day02, day03, day04, day05, day06, day07, day08, day09};
use aoc2023::utils::{pretty_print_part_one, pretty_print_part_two};
use aoc2023::utils::get_day_input;

const ADVENT_OF_CODE_DOT_COM: &'static str = "https://adventofcode.com/2023/day/";

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<_>>();

    launch_day(&args, 1, day01::part_one, day01::part_two)?;
    launch_day(&args, 2, day02::part_one, day02::part_two)?;
    launch_day(&args, 3, day03::part_one, day03::part_two)?;
    launch_day(&args, 4, day04::part_one, day04::part_two)?;
    launch_day(&args, 5, day05::part_one, day05::part_two)?;
    launch_day(&args, 6, day06::part_one, day06::part_two)?;
    launch_day(&args, 7, day07::part_one, day07::part_two)?;
    launch_day(&args, 8, day08::part_one, day08::part_two)?;
    launch_day(&args, 9, day09::part_one, day09::part_two)?;

    Ok(())
}

fn launch_day<T: std::fmt::Display>(args: &HashSet<String>, day: u8, part1: fn(&str) -> T, part2: fn(&str) -> T) -> Result<(), Error> {
    if args.is_empty() || args.contains(&format!("{:02}", day)) {
        let data = get_day_input(day)?;
        println!("--- Day {:02} ---", day);
        println!("{}{day}", ADVENT_OF_CODE_DOT_COM);
        pretty_print_part_one(|| part1(&data));
        pretty_print_part_two(|| part2(&data));
        println!();
    }
    Ok(())
}