use std::collections::HashSet;

pub fn part_one(data: &str) -> i64 {
    let galaxies = parse_data(data, 2);
    length_of_the_shortest_path_between_every_pair_of_galaxies(&galaxies)
}

pub fn part_two(data: &str) -> i64 {
    let galaxies = parse_data(data, 1_000_000);
    length_of_the_shortest_path_between_every_pair_of_galaxies(&galaxies)
}

fn length_of_the_shortest_path_between_every_pair_of_galaxies(galaxies: &Vec<(i32, i32)>) -> i64 {
    let mut total_distance: i64 = 0;
    for (index, first) in galaxies.iter().enumerate() {
        for second in &galaxies[index + 1..] {
            total_distance += manhattan_distance(first, second);
        }
    }
    total_distance
}

fn manhattan_distance(first: &(i32, i32), second: &(i32, i32)) -> i64 {
    let (x1, y1) = first;
    let (x2, y2) = second;
    ((x1 - x2).abs() + (y1 - y2).abs()) as i64
}

fn parse_data(data: &str, expansion_factor: i32) -> Vec<(i32, i32)> {
    let lines: Vec<&str> = data.lines().collect();
    let mut rows: HashSet<i32> = HashSet::new();
    let mut cols: HashSet<i32> = HashSet::new();
    let mut result: Vec<(i32, i32)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if !line.contains('#') {
            rows.insert(i as i32);
        }
    }

    let max_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    for col in 0..max_length {
        if lines.iter().all(|line| line.chars().nth(col).unwrap_or_default() != '#') {
            cols.insert(col as i32);
        }
    }

    let mut expanded_y = -1;
    for (y, row) in lines.iter().enumerate() {
        let mut expanded_x = -1;
        expanded_y += if rows.contains(&(y as i32)) { expansion_factor } else { 1 };
        for (x, char) in row.chars().enumerate() {
            expanded_x += if cols.contains(&(x as i32)) { expansion_factor } else { 1 };
            if char == '#' {
                result.push((expanded_y, expanded_x));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part_one_examples() {
        let training: &str = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        assert_eq!(374, part_one(training));
    }
}



