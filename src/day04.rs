use std::cmp::max;
use std::collections::HashSet;

pub fn part_one(data: &str) -> u32 {
    let cards = build_cards(data);

    cards.iter().map(|card| card.points()).sum()
}

pub fn part_two(data: &str) -> u32 {
    let cards = build_cards(data);

    // Create a Vec from a given element and size
    let mut counts = vec![1; cards.len()];
    // iterate over the cards
    for (index, card) in cards.iter().enumerate() {
        // iterate over the winning numbers
        for i in 1..=card.winning() {
            // if the index + i is less than the number of cards
            if index + i < cards.len() {
                // add the count of the current card to the count of the card at index + i
                counts[index + i] += counts[index];
            }
        }
    }
    //
    counts.iter().sum()
}

/// build_cards method parses the input lines and creates Card instances.
fn build_cards(data: &str) -> Vec<Card> {
    let lines = data.split_terminator('\n').collect::<Vec<_>>();
    lines.iter().map(|line| {
        let parts: Vec<&str> = line.split(':').collect();
        // parts[0] is the card number
        // parts[1] is the winning numbers and the numbers you have
        let numbers: Vec<&str> = parts[1].split('|').collect();

        let winning_numbers: HashSet<u32> = numbers[0].trim().split_whitespace()
            .map(|number| number.parse().expect("Invalid number"))
            .collect();
        let numbers_you_have: Vec<u32> = numbers[1].trim().split_whitespace()
            .map(|number| number.parse().expect("Invalid number"))
            .collect();

        Card::new(winning_numbers, numbers_you_have)
    }).collect()
}

/// Card struct represents a card.
struct Card {
    // a HashSet to store the winning_numbers for efficient lookup
    winning_numbers: HashSet<u32>,
    // a Vec to store the numbers_you_have
    numbers_you_have: Vec<u32>,
}

/// Card implementation
impl Card {
    fn new(winning_numbers: HashSet<u32>, numbers_you_have: Vec<u32>) -> Self {
        Card {
            winning_numbers,
            numbers_you_have,
        }
    }

    /// points method returns the points for the card.
    fn points(&self) -> u32 {
        if self.winning() <= 0 {
            return 0;
        }
        2_u32.pow(max(0,self.winning() as u32 - 1))
    }

    /// winning method returns the number of winning numbers you have.
    fn winning(&self) -> usize {
        self.numbers_you_have.iter().filter(|&&num| self.winning_numbers.contains(&num)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    static TRAINING: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn part_one_examples() {
        assert_eq!(13, part_one(TRAINING));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(30, part_two(TRAINING));
    }
}
