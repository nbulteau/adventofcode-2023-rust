use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Error;

use aoc2023::{day01, day02, day03, day04};
use aoc2023::utils::{pretty_print_part_one, pretty_print_part_two};
use aoc2023::utils::get_day_input;

const ADVENT_OF_CODE_DOT_COM: &'static str = "https://adventofcode.com/2023/day/";

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<HashSet<_>>();

    launch_day(&args, 1, day01::part_one, day01::part_two)?;
    launch_day(&args, 2, day02::part_one, day02::part_two)?;
    launch_day(&args, 3, day03::part_one, day03::part_two)?;
    launch_day(&args, 4, day04::part_one, day04::part_two)?;

    Ok(())
}

fn launch_day(args: &HashSet<String>, day: u8, part1: fn(&str) -> u32, part2: fn(&str) -> u32) -> Result<(), Error> {
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