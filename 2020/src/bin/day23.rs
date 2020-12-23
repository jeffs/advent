#![allow(dead_code, unused_variables)]

const BASE: usize = 10;

type Cup = u8;
type CupArray = [Cup; BASE - 1];

fn has_all_digits(cups: CupArray) -> bool {
    (1..BASE).all(|digit| cups.iter().any(|&cup| cup as usize == digit))
}

struct Circle {
    cups: CupArray,
}

impl Circle {
    fn new(mut digits: u64) -> Circle {
        assert!(digits < BASE.pow(BASE as u32) as u64);
        let mut cups = [0; BASE - 1];
        for i in (0..(BASE - 1)).rev() {
            cups[i] = (digits % BASE as u64) as Cup;
            digits /= BASE as u64;
        }
        assert!(has_all_digits(cups));
        Circle { cups }
    }
}

fn solve(digits: u64, moves: usize) -> u64 {
    let circle = Circle::new(digits);
    todo!()
}

fn main() {
    println!("{}", solve(624397158, 100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1() {
        assert_eq!(92658374, solve(389125467, 10));
        assert_eq!(67384529, solve(389125467, 100));
    }
}
