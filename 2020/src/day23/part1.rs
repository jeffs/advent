const BASE: usize = 10;
const BASE_U64: u64 = BASE as u64;

type Cup = u8;
type CupArray = [Cup; BASE - 1];

fn has_all_digits(cups: CupArray) -> bool {
    (1..BASE).all(|digit| cups.iter().any(|&cup| cup == digit as Cup))
}

#[derive(Clone)]
struct Circle {
    cups: CupArray,
}

impl Circle {
    fn from_digits(mut digits: u64) -> Circle {
        assert!(digits < BASE_U64.pow(BASE as u32));
        let mut cups = [0; BASE - 1];
        for i in 1..BASE {
            cups[BASE - 1 - i] = (digits % BASE_U64) as Cup;
            digits /= BASE_U64;
        }
        assert!(has_all_digits(cups));
        Circle { cups }
    }

    fn as_answer(&self) -> u64 {
        let mut cups = self.cups;
        while cups[0] != 1 {
            cups.rotate_left(1);
        }
        cups[1..]
            .iter()
            .fold(0, |u, &cup| u * BASE_U64 + cup as u64)
    }

    fn next(self) -> Circle {
        const LEN: usize = BASE - 1; // number of cups
        const WINDOW: usize = 3;
        let mut cups = self.cups;
        cups[1..].rotate_left(WINDOW);
        let destination_index = (1..=BASE)
            .find_map(|subtrahend| {
                let label = (cups[0] as usize + BASE - subtrahend) % BASE;
                cups[..(LEN - WINDOW)]
                    .iter()
                    .position(|&cup| cup == label as Cup)
            })
            .unwrap();
        cups[(destination_index + 1)..].rotate_right(WINDOW);
        cups.rotate_left(1);
        Circle { cups }
    }

    fn nth(self, n: usize) -> Circle {
        (0..n).fold(self, |circle, _| circle.next())
    }
}

pub fn solve(digits: u64) -> u64 {
    Circle::from_digits(digits).nth(100).as_answer()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: u64 = 389125467;

    #[test]
    fn as_answer_sample1() {
        assert_eq!(92658374, Circle::from_digits(583741926).as_answer());
    }

    #[test]
    fn nth_sample1() {
        let circle = Circle::from_digits(SAMPLE1);
        assert_eq!(92658374, circle.clone().nth(10).as_answer());
    }

    #[test]
    fn solve_sample1() {
        assert_eq!(67384529, solve(SAMPLE1));
    }
}
