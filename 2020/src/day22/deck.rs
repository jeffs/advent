use crate::error::ParseError;
use std::collections::VecDeque;

pub type Card = u8;
pub type Deck = VecDeque<Card>; // ordered top to bottom

pub fn parse_both(text: String) -> Result<(Deck, Deck), ParseError> {
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

pub fn score(deck: Deck) -> u64 {
    deck.iter()
        .enumerate()
        .map(|(i, &card)| (deck.len() - i) as u64 * card as u64)
        .sum()
}
