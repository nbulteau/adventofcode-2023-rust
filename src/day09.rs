pub fn part_one(data: &str) -> i32 {
    let lists = extract_lists(data);
    return lists.iter().map(|list| next_value(&list)).sum();
}

pub fn part_two(data: &str) -> i32 {
    let lists = extract_lists(data);
    return lists.iter().map(|list| {
        // Reverse the list so we can use the same algorithm as part one
        let mut reversed_list = list.clone();
        reversed_list.reverse();
        next_value(&reversed_list)
    }).sum();
}

fn next_value(values: &[i32]) -> i32 {
    if values.iter().all(|&value| value == 0) {
        0
    } else {
        let differences: Vec<i32> = values.windows(2).map(|window| window[1] - window[0]).collect();
        values.last().unwrap() + next_value(&differences)
    }
}

fn extract_lists(data: &str) -> Vec<Vec<i32>> {
    data.lines().map(|line| {
        line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
    }).collect()
}
#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part_one_examples() {
        let training: &str = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};

        assert_eq!(114, part_one(training));
    }

    #[test]
    fn part_two_examples() {
        let training: &str = indoc! {"
            10 13 16 21 30 45
        "};

        assert_eq!(5, part_two(training));
    }
}



