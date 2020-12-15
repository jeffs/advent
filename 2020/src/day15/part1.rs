#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

const COUNT: usize = 2020;

type Number = usize;

// Zero-based index into the sequence of numbers.
type Time = usize;

#[derive(Clone, Copy)]
enum LastSeen {
    Once { when: Time },
    Twice { earlier: Time, later: Time },
}

struct Game {
    seen: HashMap<Number, LastSeen>,
    last: usize,
}

impl Game {
    pub fn start(numbers: &[Number]) -> Game {
        assert!(!numbers.is_empty());
        let mut seen = HashMap::new();
        for (t, &n) in numbers.iter().enumerate() {
            seen.entry(n)
                .and_modify(|last_seen| {
                    *last_seen = match last_seen {
                        LastSeen::Once { when } => {
                            assert_eq!(n, numbers[*when]);
                            LastSeen::Twice {
                                earlier: *when,
                                later: t,
                            }
                        }
                        LastSeen::Twice { earlier, later } => {
                            assert_eq!(n, numbers[*earlier]);
                            assert_eq!(n, numbers[*later]);
                            LastSeen::Twice {
                                earlier: *later,
                                later: t,
                            }
                        }
                    }
                })
                .or_insert(LastSeen::Once { when: t });
        }
        Game {
            seen,
            last: numbers[numbers.len() - 1],
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Finds the 2020th _number_, starting from the specified _starting numbers_
/// and proceeding according to the rules of the elves' _memory game_.
pub fn solve(starting_numbers: &[usize]) -> usize {
    Game::start(starting_numbers).nth(COUNT - starting_numbers.len() - 1).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample036() {
        assert_eq!(436, solve(&[0, 3, 6]));
    }

    // Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    // Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    // Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    // Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    // Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    // Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
}
