use std::collections::VecDeque;

const WINDOW: usize = 3;

type Cup = u32;

/// Wraps around to the specified max value, skipping 0.  For example, goes
/// from 3 to 2, from 2 to 1, and from 1 to max.
fn decrement(label: Cup, max: Cup) -> Cup {
    let label = label - 1;
    ((label + max - 1) % max + 1) as Cup
}

fn destination(current: Cup, removed: &[Cup; WINDOW], max: Cup) -> Cup {
    let mut dest = decrement(current, max);
    while removed.contains(&dest) {
        dest = decrement(dest, max)
    }
    dest
}

fn has_all_labels(cups: &[Cup], max: Cup) -> bool {
    let mut cups = cups.to_vec();
    cups.sort_unstable();
    (1..=max).eq(cups.iter().cloned())
}

pub struct Circle {
    cups: VecDeque<Cup>,
    round: usize,
}

impl Circle {
    fn new(mut digits: u64, max: Cup) -> Circle {
        // Input is base 10, but cup labels go much higheer.
        let mut cups = VecDeque::new();
        cups.reserve(max as usize);
        while digits > 0 {
            cups.push_front((digits % 10) as Cup);
            digits /= 10;
        }
        cups.extend((cups.len() as Cup + 1)..=max);
        assert!(has_all_labels(cups.make_contiguous(), max));
        Circle { cups, round: 0 }
    }

    pub fn solve1(digits: u64, max: Cup, count: usize) -> u64 {
        Circle::new(digits, max).nth(count).into_answer1()
    }

    pub fn solve2(digits: u64, max: Cup, count: usize) -> u64 {
        Circle::new(digits, max).nth(count).into_answer2()
    }

    fn insert_after(&mut self, mut index: usize, cups: [Cup; WINDOW]) {
        // TODO: Move all disturbed cups back by the WINDOW length a priori,
        // rather than shifting all disturbed cups repeated for each inserted
        // one.

        if index + WINDOW < cups.len() {
            // Grow the deque, push cups out of the way, and insert the new ones.
            self.cups.extend(cups.iter().cloned());
            for i in ((index + 1 + WINDOW)..self.cups.len()).rev() {
                self.cups[i] = self.cups[i - WINDOW];
            }
            for &cup in &cups {
                index += 1;
                self.cups[index] = cup;
            }
        } else {
            for &cup in &cups {
                index += 1;
                self.cups.insert(index, cup);
            }
        }
    }

    /// Moves the current (first) cup to the back of the queue, then removes
    /// and returns WINDOW cups.
    fn shift(&mut self) -> [Cup; WINDOW] {
        self.cups.rotate_left(1);
        let mut removed: [Cup; WINDOW] = [0; WINDOW];
        for r in &mut removed {
            *r = self.cups.pop_front().unwrap();
        }
        removed
    }

    fn next(mut self) -> Circle {
        if self.round % 10000 == 0 {
            println!("round {}", self.round);
        }
        self.round += 1;
        let max = self.cups.len() as Cup;
        let current = self.cups[0];
        let removed = self.shift();
        let dest = destination(current, &removed, max);
        let pos = self.cups.iter().position(|&cup| cup == dest).unwrap();
        self.insert_after(pos, removed);
        self
    }

    fn nth(self, n: usize) -> Circle {
        (0..n).fold(self, |circle, _| circle.next())
    }

    fn into_answer1(mut self) -> u64 {
        while self.cups[0] != 1 {
            self.cups.rotate_left(1);
        }
        self.cups
            .iter()
            .skip(1)
            .fold(0, |u, &cup| u * 10 + cup as u64)
    }

    fn into_answer2(&self) -> u64 {
        let length = self.cups.len();
        let index = self.cups.iter().position(|&cup| cup == 1).unwrap();
        let multiplicand = self.cups[(index + 1) % length] as u64;
        let multiplier = self.cups[(index + 2) % length] as u64;
        multiplicand * multiplier
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: u64 = 389125467;

    #[test]
    fn into_answer1() {
        assert_eq!(92658374, Circle::new(583741926, 9).into_answer1());
    }

    #[test]
    fn solve1_sample1() {
        assert_eq!(92658374, Circle::solve1(SAMPLE1, 9, 10));
        assert_eq!(67384529, Circle::solve1(SAMPLE1, 9, 100));
    }

    #[test]
    fn solve2_sample1() {
        assert_eq!(149245887792, Circle::solve2(SAMPLE1, 999_999, 10_000_000));
    }
}
