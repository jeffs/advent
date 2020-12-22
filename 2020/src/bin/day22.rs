use advent2020::error::ParseError;
use std::collections::VecDeque;
use std::fs;

type Card = u8;
type Deck = VecDeque<Card>; // ordered top to bottom

fn round(mut decks: (Deck, Deck)) -> Option<(Deck, Deck)> {
    match (decks.0.pop_front(), decks.1.pop_front()) {
        (Some(a), Some(b)) => {
            if a > b {
                decks.0.extend(&[a, b])
            } else {
                decks.1.extend(&[b, a])
            }
            Some(decks)
        }
        _ => None,
    }
}

fn play(mut decks: (Deck, Deck)) -> (Deck, Deck) {
    while !(decks.0.is_empty() || decks.1.is_empty()) {
        decks = round(decks).unwrap();
    }
    decks
}

fn score(deck: Deck) -> u64 {
    deck.iter()
        .enumerate()
        .map(|(i, &card)| (deck.len() - i) as u64 * card as u64)
        .sum()
}

fn parse(text: String) -> Result<(Deck, Deck), ParseError> {
    let mut decks = (Deck::new(), Deck::new());
    let mut lines = text.lines().skip(1);
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        decks.0.push_back(line.parse()?);
    }
    for line in lines.skip(1) {
        if line.is_empty() {
            break;
        }
        decks.1.push_back(line.parse()?);
    }
    Ok(decks)
}

fn solve_part1(decks: (Deck, Deck)) -> u64 {
    let decks = play(decks);
    let winner = if decks.0.is_empty() { decks.1 } else { decks.0 };
    score(winner)
}

fn main() {
    let input_path = "tests/day22/input";
    let decks = parse(fs::read_to_string(input_path).unwrap()).unwrap();
    println!("{}", solve_part1(decks));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_sample1() {
        let input_path = "tests/day22/sample1";
        let decks = parse(fs::read_to_string(input_path).unwrap()).unwrap();
        assert_eq!(306, solve_part1(decks));
    }
}
