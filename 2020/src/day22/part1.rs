use super::{deck, Deck};

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

pub fn solve(decks: (Deck, Deck)) -> u64 {
    let decks = play(decks);
    let winner = if decks.0.is_empty() { decks.1 } else { decks.0 };
    deck::score(winner)
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
        assert_eq!(306, solve(decks));
    }
}
