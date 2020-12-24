//! I struggled with part 2, trying to update the algo from part 1.  Finally,
//! and for the first time ever, I looked at somebody else's Advent code before
//! submitting a solution, and found it extremely helpful.  Thank you, Jonas
//! Karlsson:
//! https://github.com/karjonas/advent-of-code/blob/master/2020/day23/src/lib.rs
//!
//! The trick is to view the Circle as a directed graph in which each cup is a
//! node having exactly one child---its neighbor, going clockwise---and to
//! represent that graph using an adjacency matrix.  Because the graph is so
//! simple (each node having exactly one child), the "matrix" is simply an
//! array.
//!
//! The performance of this approach is much better than something naive (like
//! the VecDeque I tried earlier) for a couple of reasons:
//!
//! * We don't have to do a linear scan for the destination cup, which would
//!   take O(N) comparisons on every move.
//!
//!   Cup labels are consecutive integers, so they can be indexes into the
//!   adjacency array.  That blew my mind: I was trying to preserve the order
//!   of the cups, but we really don't care about the order except to know any
//!   given cup's clockwise neighbor.  Each cup can sit forever at a fixed
//!   index.  It's a cleverly framed problem.
//!
//! * Most cups' neighbors do not change on any given move, so we don't
//!   have to touch them.  Inserting cups in the middle of a deque means
//!   pushing back thousands of subsequent cups, even when they had nothing to
//!   do with the move.

const WINDOW: usize = 3; // number of cups moved at a time

type Cup = u32;

/// A Circle is conceptually a circular linked list, and as such a trivial
/// graph.  It represents that graph as a single-row adjacency matrix, in which
/// indexes are cup label values (minus 1 so we can use 0-based indexes).
pub struct Circle {
    adjacent: Vec<Cup>, // maps cups (indexes) to their clockwise neighbors
    current: Cup,
}

impl Circle {
    fn from_digits(mut digits: u64, len: usize) -> Circle {
        let mut prefix = Vec::new(); // in reverse order
        prefix.reserve(9);
        while digits > 0 {
            prefix.push((digits % 10) as Cup - 1);
            digits /= 10;
        }
        let mut adjacent = Vec::new();
        adjacent.reserve(len);
        adjacent.resize(prefix.len(), 0);
        for (i, &cup) in prefix.iter().enumerate() {
            adjacent[cup as usize] = prefix[(i + prefix.len() - 1) % prefix.len()];
        }
        let mut last = prefix[0];
        while adjacent.len() < len {
            let cup = adjacent.len() as Cup;
            adjacent[last as usize] = cup;
            adjacent.push(cup);
            last = cup;
        }
        let current = prefix[prefix.len() - 1];
        adjacent[last as usize] = current;
        Circle { adjacent, current }
    }

    fn remove(&mut self) -> [Cup; WINDOW] {
        let mut removed = [0; WINDOW];
        let mut cup = self.current;
        for i in 0..WINDOW {
            cup = self.adjacent[cup as usize];
            removed[i] = cup;
        }
        self.adjacent[self.current as usize] = self.adjacent[cup as usize];
        removed
    }

    fn destination(&self, removed: &[Cup; WINDOW]) -> Cup {
        let len = self.adjacent.len();
        let mut cup = ((self.current as usize + len - 1) % len) as Cup;
        while removed.contains(&cup) {
            cup = ((cup as usize + len - 1) % len) as Cup;
        }
        cup
    }

    fn next(mut self) -> Circle {
        let removed = self.remove();
        let dest = self.destination(&removed);
        self.adjacent[removed[WINDOW - 1] as usize] = self.adjacent[dest as usize];
        self.adjacent[dest as usize] = removed[0];
        self.current = self.adjacent[self.current as usize];
        self
    }

    fn nth(self, n: usize) -> Circle {
        (0..n).fold(self, |circle, _| circle.next())
    }

    fn as_answer(&self) -> u64 {
        let multiplicand = self.adjacent[0] as u64;
        let multiplier = self.adjacent[multiplicand as usize] as u64;
        (multiplicand + 1) * (multiplier + 1)
    }

    /// Produces an answer of the form required for part 1.
    fn as_answer1(&self) -> u64 {
        let mut answer = 0;
        let mut cup: Cup = 0;
        for _ in 1..self.adjacent.len() {
            cup = self.adjacent[cup as usize];
            answer = answer * 10 + cup as u64 + 1
        }
        answer
    }
}

pub fn solve(digits: u64) -> u64 {
    Circle::from_digits(digits, 1_000_000)
        .nth(10_000_000)
        .as_answer()
}

/// Solves part 1 using the Circle implementation from part 2.  There are two
/// big differences between part 1 and part 2:
///
/// 1. The sheer size of part 2 requires dynamic allocation and a clever
///    algorithm.  Putting million-element arrays on the stack quickly
///    overflows it.
///
/// 2. The circle for part 2 supports automatic insertion of more cups than
///    were explicitly specified in the input.
pub fn solve1(digits: u64) -> u64 {
    Circle::from_digits(digits, 9).nth(100).as_answer1()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: u64 = 389125467;

    #[test]
    fn as_answer1_sample1() {
        assert_eq!(92658374, Circle::from_digits(583741926, 9).as_answer1());
    }

    #[test]
    fn solve1_sample1() {
        assert_eq!(67384529, solve1(SAMPLE1));
    }

    #[test]
    fn solve_sample1() {
        assert_eq!(149245887792, solve(SAMPLE1));
    }
}
