use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day7.txt").expect("Failed to read input");
    let hands = read_hands(&input);
    let total_winnings = get_winnings(hands);
    println!("Total winnigs: {}", total_winnings);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    bid: u32,
    cards: String,
    kind: HandKind,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            return Ordering::Equal;
        }

        let mut candidate = self.kind.cmp(&other.kind);
        let mut self_cards = self.cards.chars();
        let mut other_cards = other.cards.chars();

        while candidate == Ordering::Equal {
            let self_high_value = get_card_value(self_cards.next().unwrap());
            let other_high_value = get_card_value(other_cards.next().unwrap());
            candidate = self_high_value.cmp(&other_high_value);
        }
        candidate
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: String, bid: u32) -> Hand {
        let kind = Hand::categorize(&Hand::card_count(&cards));

        Hand { bid, cards, kind }
    }

    fn categorize(char_count: &HashMap<char, u32>) -> HandKind {
        let counts: Vec<u32> = char_count.values().cloned().collect();

        if counts.contains(&5) {
            HandKind::FiveOfAKind
        } else if counts.contains(&4) {
            HandKind::FourOfAKind
        } else if counts.contains(&3) && counts.contains(&2) {
            HandKind::FullHouse
        } else if counts.contains(&3) {
            HandKind::ThreeOfAKind
        } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
            HandKind::TwoPair
        } else if counts.contains(&2) {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }

    fn card_count(cards: &str) -> HashMap<char, u32> {
        let mut char_count = HashMap::new();
        let mut num_jokers = 0;

        for c in cards.chars() {
            if c == 'J' {
                num_jokers += 1;
            } else {
                let count = char_count.entry(c).or_insert(0);
                *count += 1;
            }
        }

        // Find the key with the max count
        // Special edge case for all 5 Jokers
        if char_count.len() == 0 {
            char_count.insert('J', 5);
        } else {
            let max_entry = char_count.iter().max_by_key(|x| x.1).unwrap();
            let max_count = char_count.entry(*max_entry.0).or_insert(0);
            *max_count += num_jokers;
        }

        char_count
    }
}

fn read_hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|hand| {
            let parts: Vec<&str> = hand.split_whitespace().collect();
            let cards = parts[0];
            let bid: u32 = parts[1].parse().unwrap();
            Hand::new(cards.to_string(), bid)
        })
        .collect()
}

fn get_card_value(card: char) -> u8 {
    match card {
        '0'..='9' => card.to_digit(10).unwrap() as u8,
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn get_winnings(mut hands: Vec<Hand>) -> u32 {
    hands.sort();

    let mut total_winnings: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i as u32 + 1;
        total_winnings += hand.bid * rank;
    }

    total_winnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    fn test_read_hands() {
        let hands = read_hands(INPUT);
        assert_eq!(hands.len(), 5);

        assert_eq!(hands[0].cards, "32T3K");
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[0].kind, HandKind::OnePair);

        assert_eq!(hands[1].cards, "T55J5");
        assert_eq!(hands[1].bid, 684);
        assert_eq!(hands[1].kind, HandKind::ThreeOfAKind);

        assert_eq!(hands[2].cards, "KK677");
        assert_eq!(hands[2].bid, 28);
        assert_eq!(hands[2].kind, HandKind::TwoPair);

        assert_eq!(hands[3].cards, "KTJJT");
        assert_eq!(hands[3].bid, 220);
        assert_eq!(hands[3].kind, HandKind::TwoPair);

        assert_eq!(hands[4].cards, "QQQJA");
        assert_eq!(hands[4].bid, 483);
        assert_eq!(hands[4].kind, HandKind::ThreeOfAKind);
    }

    #[test]
    fn test_compare() {
        let mut hands = read_hands(INPUT);

        hands.sort();

        assert_eq!(hands[4].bid, 220);
        assert_eq!(hands[3].bid, 483);
        assert_eq!(hands[2].bid, 684);
        assert_eq!(hands[1].bid, 28);
        assert_eq!(hands[0].bid, 765);
    }

    #[test]
    fn test_winnings() {
        let hands = read_hands(INPUT);
        let total_winnings = get_winnings(hands);
        assert_eq!(total_winnings, 5905);
    }
}
