use crate::card::Card;

pub fn solve(text: &str) -> usize {
    let cards: Vec<Card> = text.lines().map(Card::from_line).collect();
    let mut counts = vec![1; cards.len()];
    for card in &cards {
        for other in &cards[card.id..card.id + card.count] {
            counts[other.id - 1] += counts[card.id - 1];
        }
    }
    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 30);
    }
}
