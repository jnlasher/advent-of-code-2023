use std::collections::HashSet;
use std::fmt;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./inputs/day4.txt").expect("Failed to read input");

    let cards: Vec<Card> = scan_cards(&input);
    let total_number_cards = count_cards(cards);
    println!("Total cards: {}", total_number_cards);
}

fn scan_cards(input: &str) -> Vec<Card> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn count_cards(cards: Vec<Card>) -> u32 {
    let mut n_cards: Vec<u32> = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let new_cards = card.calculate_points();
        for _ in 0..n_cards[i] {
            for j in 1..=new_cards as usize {
                n_cards[j + i] += 1;
            }
        }
    }

    n_cards.iter().sum()
}

struct Card {
    card_id: String,
    winning_numbers: HashSet<u32>,
    game_numbers: HashSet<u32>,
}

impl Card {
    pub fn new(card_id: &str, winning_numbers: HashSet<u32>, game_numbers: HashSet<u32>) -> Card {
        Card {
            card_id: card_id.to_string(),
            winning_numbers,
            game_numbers,
        }
    }

    pub fn calculate_points(&self) -> u32 {
        let n_intersect = self.winning_numbers.intersection(&self.game_numbers);
        n_intersect.count() as u32
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nWinning Numbers: {:?}\nGame Numbers: {:?}",
            self.card_id, self.winning_numbers, self.game_numbers
        )
    }
}

fn parse_line(line: &str) -> Card {
    // Get the card number
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        panic!("Failed to parse card number for: {}", line);
    }

    let card_id = parts[0];
    let card_values = parts[1];

    // Split the line by "|"
    let parts: Vec<&str> = card_values.split('|').collect();
    let mut winning_numbers = HashSet::new();
    let mut game_numbers = HashSet::new();

    if parts.len() != 2 {
        panic!("Failed to parse value information for: {}", line);
    }

    // Parse numbers on the left side
    for num_str in parts[0].trim().split_whitespace() {
        if let Ok(num) = num_str.parse::<u32>() {
            winning_numbers.insert(num);
        }
    }

    // Parse numbers on the right side
    for num_str in parts[1].trim().split_whitespace() {
        if let Ok(num) = num_str.parse::<u32>() {
            game_numbers.insert(num);
        }
    }

    Card::new(card_id, winning_numbers, game_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_compute_points() {
        let cards: Vec<Card> = scan_cards(INPUT);
        let total_number_cards = count_cards(cards);
        assert_eq!(total_number_cards, 30);
    }
}
