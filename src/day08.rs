use std::collections::HashMap;

pub fn part_one(data: &str) -> u64 {
    let (instructions, network) = extract_data(data);
    return count_steps("AAA".to_string(), &network, &instructions, |s| s != "ZZZ");
}

pub fn part_two(data: &str) -> u64 {
    let (instructions, network) = extract_data(data);
    let start_positions: Vec<String> = network.keys().filter(|k| k.ends_with('A')).cloned().collect();
    let end_positions: Vec<u64> = start_positions.iter().map(|start| {
        count_steps(start.to_string(), &network, &instructions, |s| !s.ends_with('Z'))
    }).collect();

    return lcm_list(&end_positions);
}

fn count_steps<F>(start: String, network: &HashMap<String, (String, String)>, instructions: &Vec<char>, condition: F) -> u64
    where
        F: Fn(String) -> bool,
{
    let mut current = start;
    let mut steps = 0;
    while condition(current.clone()) {
        current = match network.get(&current) {
            Some(route) => {
                let instruction = instructions[steps % instructions.len()];
                steps += 1;
                if instruction == 'L' {
                    route.0.clone()
                } else {
                    route.1.clone()
                }
            },
            None => panic!("No such key in route_map: {}", current),
        };
    }
    return steps as u64;
}

fn extract_data(data: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let parts: Vec<&str> = data.split("\n\n").collect();
    let instructions: Vec<char> = parts[0].chars().collect();
    let network: HashMap<String, (String, String)> = parts[1].lines().map(|line| {
        let parts: Vec<&str> = line.split(" = ").collect();
        let name = parts[0].to_string();
        let values: Vec<&str> = parts[1].split(", ").collect();
        let left = values[0].trim_start_matches('(').to_string();
        let right = values[1].trim_end_matches(')').to_string();
        (name, (left, right))
    }).collect();

    return (instructions, network);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_list(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |a, &b| lcm(a, b))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn part_one_examples_one() {
        let training: &str = indoc! {"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(6440, part_one(training));
    }

    #[test]
    fn part_one_examples_two() {
        let training: &str = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        "};

        assert_eq!(6440, part_one(training));
    }

    #[test]
    fn part_two_examples() {
        let training: &str = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        "};

        assert_eq!(5905, part_two(training));
    }
}



