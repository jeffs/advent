//! I struggled with Part 2, trying to update the algo from part 1.  Finally,
//! and for the first time ever, I looked at somebody else's Advent code before
//! submitting a solution, and found it extremely helpful.  Thank you, Jonas
//! Karlsson:
//! https://github.com/karjonas/advent-of-code/blob/master/2020/day23/src/lib.rs
//!
//! The trick is to view the Circle as a directed graph in which each cup is a
//! node having exactly one child---its right-most neighbor---and to represent
//! that graph using an adjacency matrix.  Because the graph is so simple (each
//! node having exactly one child), the "matrix" is simply an array.
//!
//! The performance of this approach is much better than something naive (like
//! the VecDeque I tried earlier) for a couple of reasons:
//!
//! 1. We don't have to do a linear scan for the destination cup, which would
//!    take O(N) comparisons on every move.
//!
//!    Cup labels are consecutive integers, so they can be indexes into the
//!    "right neighbor" array (i.e., the adjacency matrix).  That blew my mind:
//!    I was trying to preserve the order of the cups, but we really don't care
//!    about the order except to know any given cup's right-most neighbor.
//!    Each cup can sit forever at a fixed index.  It's a cleverly framed
//!    problem.
//!
//! 2. Most cups' neighbors do not change on any given move, so we don't
//!    have to touch them.  Inserting cups at arbitrary destinations in a deque
//!    means pushing back all (~500K) subsequent cups, even when they had
//!    nothing to do with the move.

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

/// A Circle is conceptually a circular linked list, and as such a trivial
/// graph.  It represents that graph as a single-row adjacency matrix, in which
/// indexes are cup label values (minus 1 so we can use 0-based indexes) an
pub struct Circle {
    adjacent: Vec<Cup>,
    round: usize,
}

impl Circle {
    fn new(mut digits: u64, max: Cup) -> Circle {
        let mut neighbors = VecDeque::new();
        cups.reserve(max as usize - 1);
        // Input is base 10, but cup labels go much higher.
        while digits > 0 {
            cups.push_front((digits % 10) - 1 as Cup);
            digits /= 10;
        }
        cups.extend((cups.len() as Cup + 1)..=max);
        debug_assert!(has_all_labels(cups.make_contiguous(), max));
        Circle { cups, round: 0 }
    }

    pub fn solve1(digits: u64, max: Cup, count: usize) -> u64 {
        Circle::new(digits, max).nth(count).into_answer1()
    }

    // 733041926064 is too high
    // 366520963032 is too high
    // 183260481516 is too low
    // 274890722274 is incorrect, and now I have to wait 5 minutes
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
            let prefix: Vec<_> = self.cups.iter().take(12).collect();
            println!("round {}: {:?}", self.round, prefix);
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
        let prefix: Vec<_> = self.cups.iter().take(12).collect();
        println!("round {}: {:?}", self.round, prefix);
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
    fn into_answer2() {
        assert_eq!(18, Circle::new(583741926, 9).into_answer2());
    }

    #[test]
    fn solve1_sample1() {
        assert_eq!(92658374, Circle::solve1(SAMPLE1, 9, 10));
        assert_eq!(67384529, Circle::solve1(SAMPLE1, 9, 100));
    }

    #[test]
    fn solve1_answer2() {
        assert_eq!(18, Circle::new(SAMPLE1, 9).nth(10).into_answer2());
    }

    #[test]
    fn solve2_sample1() {
        assert_eq!(149245887792, Circle::solve2(SAMPLE1, 999_999, 10_000_000));
    }
}
