use crate::puzzle::Puzzle;
use crate::wrap;

struct Die {
    next: usize,
}

impl Die {
    pub fn new() -> Die {
        Die { next: 1 }
    }

    pub fn roll(&mut self) -> usize {
        let next = wrap::inc(self.next, 100);
        std::mem::replace(&mut self.next, next)
    }

    pub fn roll_sum(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.roll()).sum()
    }
}

pub fn solve(puzzle: &Puzzle) -> usize {
    let mut die = Die::new();
    let mut players = puzzle.new_players();
    players[0].advance(die.roll_sum(3));
    let mut i = 1; // number of turns taken so far
    while players[(i - 1) % 2].score < 1000 {
        players[i % 2].advance(die.roll_sum(3));
        i += 1;
    }
    let score = players[i % 2].score;
    let rolls = i * 3;
    score * rolls
}

#[cfg(test)]
mod tests {
    use super::{solve, Puzzle};

    #[test]
    fn test_solve() {
        assert_eq!(739785, solve(&Puzzle::new(4, 8)));
    }
}
