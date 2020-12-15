use std::collections::HashMap;

// Something an elf says, apparently.
type Number = usize;

// Turn number; i.e., 1-based index into the sequence of numbers.
type Time = usize;

struct Game {
    seen: HashMap<Number, Time>,
    last: Number,
    time: Time,
}

impl Game {
    pub fn start(numbers: &[Number]) -> Game {
        assert!(!numbers.is_empty());
        Game {
            seen: numbers
                .iter()
                .take(numbers.len() - 1)
                .enumerate()
                .map(|(t, &n)| (n, t + 1))
                .collect(),
            last: numbers[numbers.len() - 1],
            time: numbers.len(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.time += 1;
        let key = self.last;
        self.last = if !self.seen.contains_key(&key) {
            // “If that was the first time the number has been spoken, the
            // current player says 0.”
            0
        } else {
            // “Otherwise, the number had been spoken before; the current
            // player announces how many turns apart the number is from when it
            // was previously spoken.”
            self.time - 1 - self.seen[&key]
        };
        self.seen.insert(key, self.time - 1);
        Some(self.last)
    }
}

/// Finds the 2020th number, starting from the specified starting numbers and
/// proceeding according to the rules of the elves' memory game.
pub fn solve(starting_numbers: &[usize]) -> usize {
    const COUNT: usize = 2020;
    Game::start(starting_numbers)
        .nth(COUNT - starting_numbers.len() - 1)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game036_next() {
        let mut game = Game::start(&[0, 3, 6]);
        assert_eq!(Some(0), game.next()); // Turn  4
        assert_eq!(Some(3), game.next()); // Turn  5
        assert_eq!(Some(3), game.next()); // Turn  6
        assert_eq!(Some(1), game.next()); // Turn  7
        assert_eq!(Some(0), game.next()); // Turn  8
        assert_eq!(Some(4), game.next()); // Turn  9
        assert_eq!(Some(0), game.next()); // Turn 10
    }

    #[test]
    fn solve_samples() {
        assert_eq!(436, solve(&[0, 3, 6]));
        assert_eq!(1, solve(&[1, 3, 2]));
        assert_eq!(10, solve(&[2, 1, 3]));
        assert_eq!(27, solve(&[1, 2, 3]));
        assert_eq!(78, solve(&[2, 3, 1]));
        assert_eq!(438, solve(&[3, 2, 1]));
        assert_eq!(1836, solve(&[3, 1, 2]));
    }
}
