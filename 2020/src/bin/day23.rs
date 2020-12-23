#![allow(dead_code, unused_variables)]

const BASE: usize = 10;
const BASE_U64: u64 = BASE as u64;

type Cup = u8;
type CupArray = [Cup; BASE - 1];

fn has_all_digits(cups: CupArray) -> bool {
    (1..BASE).all(|digit| cups.iter().any(|&cup| cup == digit as Cup))
}

struct Circle {
    cups: CupArray,
}

impl Circle {
    fn new(mut digits: u64) -> Circle {
        assert!(digits < BASE_U64.pow(BASE as u32));
        let mut cups = [0; BASE - 1];
        for i in 1..BASE {
            cups[BASE - 1 - i] = (digits % BASE_U64) as Cup;
            digits /= BASE_U64;
        }
        assert!(has_all_digits(cups));
        Circle { cups }
    }

    fn as_answer(self) -> u64 {
        let mut cups = self.cups;
        while cups[0] != 1 {
            cups.rotate_left(1);
        }
        cups[1..].iter().fold(0, |u, &cup| u * BASE_U64 + cup as u64)
    }
}

fn solve(digits: u64, moves: usize) -> u64 {
    let circle = (0..moves).fold(Circle::new(digits), |circle, _| {
        circle // TODO
    });
    circle.as_answer()
}

fn main() {
    println!("{}", solve(624397158, 100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn as_answer_sample1() {
        assert_eq!(92658374, Circle::new(583741926).as_answer());
    }

    #[test]
    fn solve_sample1() {
        assert_eq!(92658374, solve(389125467, 10));
        assert_eq!(67384529, solve(389125467, 100));
    }
}
