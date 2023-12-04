// Desc: Day 2 - Advent of Code

pub fn part_one(data: &str) -> u32 {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let mut index = 0;
    data.lines()
        .filter_map(|line| -> Option<u32> {
            let mut result = 0;

            let mut set_are_ok = true;

            let sets = extract_sets(line);
            for set in sets {
                let (blue, red, green) = extract_colors(set);
                set_are_ok = blue <= blue_max && red <= red_max && green <= green_max;
                if !set_are_ok {
                    break;
                }
            }
            if set_are_ok {
                result += index + 1
            }
            index += 1;

            Some(result)
        })
        .sum()
}

pub fn part_two(data: &str) -> u32 {
    data.lines()
        .filter_map(|line| -> Option<u32> {
            let mut red_max = 0;
            let mut green_max = 0;
            let mut blue_max = 0;

            let sets = extract_sets(line);
            for set in sets {
                let (blue, red, green) = extract_colors(set);
                red_max = red_max.max(red);
                green_max = green_max.max(green);
                blue_max = blue_max.max(blue);
            }

            Some(red_max * green_max * blue_max)
        }).sum()
}


fn extract_colors(set: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let colors = set.split(", ");
    for color in colors {
        let color_vec: Vec<&str> = color.split(" ").collect();
        let color_count = color_vec[0].parse::<u32>().unwrap();
        let color_name = color_vec[1];
        match color_name {
            "red" => { red += color_count; }
            "green" => { green += color_count; }
            "blue" => { blue += color_count; }
            _ => { println!("Unknown color: {}", color_name); }
        }
    }

    return (blue, red, green);
}

fn extract_sets(line: &str) -> Vec<&str> {
    // get the game part of the line
    let start_char_index = line.find(": ").unwrap() + 2;
    let game = &line[start_char_index..];
    // split the game into sets
    game.split("; ").collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    static TRAINING: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    fn part_one_examples() {
        assert_eq!(8, part_one(TRAINING));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(2286, part_two(TRAINING));
    }
}