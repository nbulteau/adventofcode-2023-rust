#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        let mut result = 0;
        for acceleration in 0..=self.time {
            if acceleration * (self.time - acceleration) > self.distance {
                result += 1;
            }
        }
        result
    }
}

pub fn part_one(data: &str) -> u64 {
    let races = build_races(data);
    // rust doesn't have a direct equivalent to Kotlin's reduce function,
    // so product is used for multiplying the elements of an iterator
    races.iter().map(|race| race.ways_to_win()).product()
}

pub fn part_two(data: &str) -> u64 {
    let race = build_race(data);
    race.ways_to_win()
}

fn build_races(data: &str) -> Vec<Race> {
    let lines: Vec<&str> = data.lines().collect();
    // split the lines into two vectors of strings
    // then trim the strings and split them into vectors of strings
    // then parse the strings into vectors of u64
    // nth() : Returns the nth element of the iterator.
    let times: Vec<u64> = lines[0].split(':').nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let distances: Vec<u64> = lines[1].split(':').nth(1).unwrap().trim().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    times.iter().zip(distances.iter()).map(|(&time, &distance)| Race { time, distance }).collect()
}

fn build_race(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0].split(':').nth(1).unwrap().trim().replace(" ", "").parse().unwrap();
    let distance = lines[1].split(':').nth(1).unwrap().trim().replace(" ", "").parse().unwrap();

    Race { time, distance }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    static TRAINING: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    fn part_one_examples() {
        assert_eq!(288, part_one(TRAINING));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(71503, part_two(TRAINING));
    }
}



