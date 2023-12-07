use std::{convert::Infallible, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// Card scores (not the original ASCII bytes).
#[derive(Debug)]
struct Hand([u8; 5]);

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

        let joker_count = self.0.into_iter().filter(|&c| c == 0).count();

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
}

impl Eq for Hand {}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = [0; 5];
        hand.copy_from_slice(s.as_bytes());
        for card in hand.iter_mut() {
            *card = b"J23456789TQKA"
                .iter()
                .position(|c| c == card)
                .expect("valid card") as u8;
        }
        Ok(Hand(hand))
    }
}

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

#[derive(Debug)]
struct Line(Hand, i64);

impl Eq for Line {}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').expect("hand and bid");
        Ok(Line(
            hand.parse().expect("valid hand"),
            bid.parse().expect("numeric bid"),
        ))
    }
}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(text: &str) -> i64 {
    let mut lines: Vec<&str> = text.lines().collect();
    lines.sort_by_key(|line| line.parse::<Line>().expect("valid line"));
    lines
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            let rank = i as i64 + 1;
            let Line(_, bid) = s.parse().expect("valid line");
            rank * bid
        })
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_cmp() {
        let hands: [Hand; 5] = ["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"]
            .map(|cards| cards.parse().expect("infallible"));
        for (index, hand) in hands.iter().enumerate() {
            assert_eq!(hand, hand);
            for other in &hands[index + 1..] {
                assert!(hand <= other)
            }
        }
    }

    #[test]
    fn hand_type_from_hand() {
        for (cards, want) in [
            ("32T3K", HandType::OnePair),
            ("KK677", HandType::TwoPair),
            ("KTJJT", HandType::FourOfAKind),
            ("T55J5", HandType::FourOfAKind),
            ("QQQJA", HandType::FourOfAKind),
        ] {
            let hand: Hand = cards.parse().expect("infallible");
            assert_eq!(hand.hand_type(), want);
        }
    }

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 5905);
    }
}
