use std::collections::HashMap;

// Something an elf says, apparently.
type Number = usize;

// Turn number; i.e., 1-based index into the sequence of numbers.
type Time = usize;

pub struct Game {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start036_next() {
        let mut game = Game::start(&[0, 3, 6]);
        assert_eq!(Some(0), game.next()); // Turn  4
        assert_eq!(Some(3), game.next()); // Turn  5
        assert_eq!(Some(3), game.next()); // Turn  6
        assert_eq!(Some(1), game.next()); // Turn  7
        assert_eq!(Some(0), game.next()); // Turn  8
        assert_eq!(Some(4), game.next()); // Turn  9
        assert_eq!(Some(0), game.next()); // Turn 10
    }
}
