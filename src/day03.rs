// Desc: Day 3 - Advent of Code
use std::collections::HashMap;
use regex::Regex;


pub fn part1(data: &str) -> u32 {
    let map = build_map(data);

    map.values().flatten().sum()
}

pub fn part2(data: &str) -> u32 {
    let map: HashMap<(usize, usize, char), Vec<u32>> = build_map(data);

    // A gear is any '*' symbol that is adjacent to exactly two part numbers.
    map.iter()
        .filter(|&(&(_, _, symbol), _)| symbol == '*') // Filter for '*' symbols
        .filter_map(|(_, list)| if list.len() == 2 { Some(list) } else { None }) // Filter for lists of size 2
        .map(|list| list[0] * list[1]) // Multiply the two numbers
        .sum() // Sum the products
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_whitespace() && !c.is_digit(10)
}

fn build_map(data: &str) -> HashMap<(usize, usize, char), Vec<u32>> {
    let lines = data.split_terminator('\n').collect::<Vec<_>>();

    let mut map: HashMap<(usize, usize, char), Vec<u32>> = HashMap::new();
    let number_regex = Regex::new(r"\d+").unwrap();

    for (y, line) in lines.iter().enumerate() {
        for match_result in number_regex.find_iter(line) {
            let number: u32 = match_result.as_str().parse().unwrap();
            let x_start = match_result.start().saturating_sub(1);
            let x_end = match_result.end();
            // The saturating_sub method is used to prevent underflow when subtracting from usize.
            for index in y.saturating_sub(1)..=(y + 1).min(lines.len() - 1) {
                let current_line = &lines[index];
                for x in x_start..=x_end.min(current_line.len() - 1) {
                    let c = current_line.chars().nth(x).unwrap();
                    if is_symbol(c) {
                        map.entry((x, index, c)).or_insert_with(Vec::new).push(number);
                    }
                }
            }
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    static TRAINING: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn part1_examples() {
        assert_eq!(4361, part1(TRAINING));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(467835, part2(TRAINING));
    }
}