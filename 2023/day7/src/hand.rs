use crate::card::Card;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Default)]
pub struct Hand(pub [Card; 5]);

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut counts = [0; 5];
        let first_position_of = |c| {
            self.0
                .iter()
                .position(|&b| b == c)
                .expect("hand to include its own card")
        };
        for card in self.0 {
            counts[first_position_of(card)] += 1;
        }
        counts.sort();
        counts.reverse();

        let before_jokers = match counts {
            [5, 0, 0, 0, 0] => HandType::FiveOfAKind,
            [4, 1, 0, 0, 0] => HandType::FourOfAKind,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [3, 1, 1, 0, 0] => HandType::ThreeOfAKind,
            [2, 2, 1, 0, 0] => HandType::TwoPair,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        };

        let joker_count = self
            .0
            .into_iter()
            .filter(|&card| card == Card::Joker)
            .count();

        match (before_jokers, joker_count) {
            (_, 0) => before_jokers,

            (HandType::FiveOfAKind, 5) => HandType::FiveOfAKind,

            (HandType::FourOfAKind, 4) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,

            (HandType::FullHouse, 3) => HandType::FiveOfAKind,
            (HandType::FullHouse, 2) => HandType::FiveOfAKind,

            (HandType::ThreeOfAKind, 3) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,

            (HandType::TwoPair, 2) => HandType::FourOfAKind,
            (HandType::TwoPair, 1) => HandType::FullHouse,

            (HandType::OnePair, 2) => HandType::ThreeOfAKind,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,

            (HandType::HighCard, 1) => HandType::OnePair,

            _ => unreachable!(),
        }
    }

    pub fn with_jokers(mut self) -> Hand {
        for card in self.0.iter_mut() {
            *card = card.with_jokers();
        }
        self
    }

    pub fn parse(s: &str) -> Hand {
        let mut cards = s.bytes().map(Card::from_ascii);
        let mut hand = Hand::default();
        hand.0
            .fill_with(|| cards.next().expect("enough cards per hand"));
        hand
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.0.cmp(&other.0))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_cmp() {
        let hands = ["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"].map(Hand::parse);
        for (index, hand) in hands.iter().enumerate() {
            assert_eq!(hand, hand);
            for other in &hands[index + 1..] {
                assert!(hand <= other)
            }
        }
    }

    #[test]
    fn hand_cmp_with_jokers() {
        let hands = ["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"]
            .map(Hand::parse)
            .map(Hand::with_jokers);
        for (index, hand) in hands.iter().enumerate() {
            assert_eq!(hand, hand);
            for other in &hands[index + 1..] {
                assert!(hand <= other)
            }
        }
    }

    #[test]
    fn hand_type() {
        for (cards, want, want_jokers) in [
            ("32T3K", HandType::OnePair, HandType::OnePair),
            ("KK677", HandType::TwoPair, HandType::TwoPair),
            ("KTJJT", HandType::TwoPair, HandType::FourOfAKind),
            ("T55J5", HandType::ThreeOfAKind, HandType::FourOfAKind),
            ("QQQJA", HandType::ThreeOfAKind, HandType::FourOfAKind),
        ] {
            let hand = Hand::parse(cards);
            assert_eq!(hand.hand_type(), want);
            assert_eq!(hand.with_jokers().hand_type(), want_jokers)
        }
    }
}
