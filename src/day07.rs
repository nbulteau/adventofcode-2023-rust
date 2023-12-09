#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    with_joker: bool,
}

impl Ord for Hand {

    // Compare hands by hand type, then by card values
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let type_order = self.hand_type().cmp(&other.hand_type());
        if type_order != std::cmp::Ordering::Equal {
            return type_order;
        }

        for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
            let card_order = self_card.cmp(other_card);
            if card_order != std::cmp::Ordering::Equal {
                return card_order;
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts = self
            .cards
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &card| {
                *map.entry(card).or_insert(0) += 1;
                map
            });

        let joker = if self.with_joker {
            counts.get(&0).cloned().unwrap_or(0)
        } else {
            0
        };

        let mut others: Vec<_> = counts.values().cloned().collect();
        others.sort_by(|a, b| b.cmp(a));

        let primary_size = *others.get(0).unwrap_or(&0);
        let has_secondary_pair = others.get(1).map_or(false, |&count| count == 2);

        match primary_size + joker {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 => {
                if has_secondary_pair {
                    HandType::FullHouse
                } else {
                    HandType::ThreeKind
                }
            }
            2 => {
                if has_secondary_pair {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }
}

fn solve(lines: Vec<&str>, with_joker: bool) -> usize {
    let cards_order = if with_joker {
        "J23456789TQKA".to_string()
    } else {
        "23456789TJQKA".to_string()
    };

    let mut hands: Vec<Hand> = lines
        .into_iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let cards_str = iter.next().unwrap();
            let bid_str = iter.next().unwrap();
            let cards: Vec<usize> = cards_str
                .chars()
                .map(|c| cards_order.find(c).unwrap())
                .collect();
            let bid = bid_str.parse().unwrap();
            Hand {
                cards,
                bid,
                with_joker,
            }
        })
        .collect();

    hands.sort();

    hands.into_iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (index + 1) * hand.bid
    })
}

pub fn part_one(data: &str) -> usize {
    let lines: Vec<&str> = data.lines().collect();
    solve(lines, false)
}

pub fn part_two(data: &str) -> usize {
    let lines: Vec<&str> = data.lines().collect();
    solve(lines, true)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    static TRAINING: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn part_one_examples() {
        assert_eq!(6440, part_one(TRAINING));
    }

    #[test]
    fn part_two_examples() {
        assert_eq!(5905, part_two(TRAINING));
    }
}



