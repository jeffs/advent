#![allow(dead_code)]

use super::{deck, Card, Deck};
use std::collections::HashSet;

enum Winner {
    Player1(Deck),
    Player2(Deck),
}

type Round = (Deck, Deck);

fn draw(decks: &mut Round) -> Option<(Card, Card)> {
    if decks.0.is_empty() || decks.1.is_empty() {
        None
    } else {
        Some((decks.0.pop_front().unwrap(), decks.1.pop_front().unwrap()))
    }
}

fn take(deck: &Deck, count: usize) -> Deck {
    deck.iter().cloned().take(count).collect()
}

fn recurse(decks: &Round, cards: (Card, Card)) -> Option<Winner> {
    let counts = (cards.0 as usize, cards.1 as usize);
    if counts.0 > decks.0.len() || counts.1 > decks.1.len() {
        None
    } else {
        Some(play((take(&decks.0, counts.0), take(&decks.1, counts.1))))
    }
}

fn play(mut decks: Round) -> Winner {
    let mut seen: HashSet<Round> = HashSet::new();
    while !seen.contains(&decks) {
        seen.insert(decks.clone());
        if let Some(cards) = draw(&mut decks) {
            match recurse(&decks, cards) {
                Some(Winner::Player1(_)) => {
                    decks.0.extend(&[cards.0, cards.1]);
                }
                Some(Winner::Player2(_)) => {
                    decks.1.extend(&[cards.1, cards.0]);
                }
                None => {
                    if cards.0 > cards.1 {
                        decks.0.extend(&[cards.0, cards.1]);
                    } else {
                        assert!(cards.1 > cards.0);
                        decks.1.extend(&[cards.1, cards.0]);
                    }
                }
            }
        } else if decks.1.is_empty() {
            return Winner::Player1(decks.0);
        } else {
            assert!(decks.0.is_empty());
            return Winner::Player2(decks.1);
        }
    }
    Winner::Player1(decks.0)
}

pub fn solve(decks: (Deck, Deck)) -> u64 {
    use deck::score;
    let deck = match play(decks) {
        Winner::Player1(deck) => deck,
        Winner::Player2(deck) => deck,
    };
    score(deck)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day22/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        let decks = deck::parse_both(text).unwrap();
        assert_eq!(291, solve(decks));
    }
}
