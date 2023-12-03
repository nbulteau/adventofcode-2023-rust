// Desc: Day 1 - Advent of Code

fn solve(data: &str, mapping: &[(&str, u32)]) -> u32 {
    data.lines()
        // filter_map is like map, but it filters out None values
        .filter_map(|line| -> Option<u32> {
            let x = mapping
                .iter()
                // find returns the first element that matches the predicate
                .filter_map(|(s, d)| line.find(s).map(|i| (i, d)))
                // min_by_key returns the element that has the smallest value for the given key
                .min_by_key(|(i, _)| *i)?
                // .1 is the second element of the tuple
                .1;
            let y = mapping
                .iter()
                // rfind returns the last element that matches the predicate
                .filter_map(|(s, d)| line.rfind(s).map(|i| (i + s.len(), d)))
                // max_by_key returns the element that has the largest value for the given key
                .max_by_key(|(i, _)| *i)?
                // .1 is the second element of the tuple
                .1;
            Some(10 * x + y)
        })
        .sum()
}

pub fn part1(data: &str) -> u32 {
    solve(
        data,
        &[
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ],
    )
}

pub fn part2(data: &str) -> u32 {
    solve(
        data,
        &[
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    static TRAINING_1: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};
    static TRAINING_2: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part1_examples() {
        assert_eq!(142, part1(TRAINING_1));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(281, part2(TRAINING_2));
    }
}