use crate::hand::Hand;

/// Cards and bid.
#[derive(Debug)]
pub struct Line {
    pub hand: Hand,
    pub bid: i64,
}

impl Line {
    pub fn with_jokers(mut self) -> Line {
        self.hand = self.hand.with_jokers();
        self
    }

    pub fn parse(s: &str) -> Line {
        let (hand, bid) = s.split_once(' ').expect("hand and bid");
        Line {
            hand: Hand::parse(hand),
            bid: bid.parse().expect("numeric bid"),
        }
    }
}

impl Eq for Line {}

impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub trait IntoWinnings {
    fn into_winnings(self) -> i64;
}

impl<I: IntoIterator<Item = Line>> IntoWinnings for I {
    fn into_winnings(self) -> i64 {
        let mut lines: Vec<Line> = self.into_iter().collect();
        lines.sort();
        lines
            .into_iter()
            .enumerate()
            .map(|(index, Line { bid, .. })| (index as i64 + 1) * bid)
            .sum()
    }
}
