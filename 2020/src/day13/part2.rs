/// This file is a line-by-line translation of src/bin/day13_part2.py.  See
/// that file for a high-level explanation of the algorithm implemented here.
use crate::error::{NoSolution, ParseError};
use std::error::Error;
use std::fs;

fn is_prime(n: usize) -> bool {
    if n < 2 {
        false
    } else if n % 2 == 0 {
        n == 2
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

#[derive(Debug)]
struct BusDelay {
    id: usize,
    delay: usize,
}

#[derive(Debug)]
struct BusRemainder {
    id: usize,
    remainder: usize,
}

fn parse_line(line: &str) -> Vec<BusDelay> {
    line.split(',')
        .enumerate()
        .filter_map(|(d, s)| s.parse().ok().map(|id| BusDelay { id, delay: d }))
        .collect()
}

fn load_input(input_path: &str) -> Result<Vec<BusDelay>, Box<dyn Error>> {
    let text = fs::read_to_string(input_path)?;
    let line = text.split_terminator('\n').nth(1).ok_or_else(|| {
        let what = format!("{}: expected two lines", input_path);
        ParseError::new(what)
    })?;
    Ok(parse_line(line))
}

/// Returns an integer N such that multiplicand * N % bus_id == remainder.
fn find_multiplier(
    multiplicand: usize,
    bus_id: usize,
    remainder: usize,
) -> Result<usize, NoSolution> {
    for n in 0..bus_id {
        if multiplicand * n % bus_id == remainder {
            return Ok(n);
        }
    }
    Err(NoSolution)
}

fn find_timestamp(bus_remainders: &[BusRemainder]) -> Result<usize, NoSolution> {
    let product: usize = bus_remainders.iter().map(|bus| bus.id).product();
    let mut sum = 0;
    for bus in bus_remainders {
        let multiplicand = product / bus.id;
        let multiplier = find_multiplier(multiplicand, bus.id, bus.remainder)?;
        let term = multiplicand * multiplier;
        sum += term;
    }
    Ok(sum % product)
}

fn solve_buses(buses: Vec<BusDelay>) -> Result<usize, NoSolution> {
    assert!(buses.iter().all(|&BusDelay { id, .. }| is_prime(id)));
    let bus_remainders: Vec<BusRemainder> = buses
        .iter()
        .map(|bus| BusRemainder {
            id: bus.id,
            remainder: if bus.id < bus.delay {
                (bus.id * bus.delay - bus.delay) % bus.id
            } else {
                (bus.id - bus.delay) % bus.id
            },
        })
        .collect();
    find_timestamp(&bus_remainders)
}

pub fn solve(input_path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(solve_buses(load_input(input_path)?)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime() {
        let want = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let got: Vec<_> = (0..20).filter(|&i| is_prime(i)).collect();
        assert_eq!(want, got);
    }

    #[test]
    fn test_find_timestamp() {
        let bus_remainders = [
            BusRemainder {
                id: 3,
                remainder: 2,
            },
            BusRemainder {
                id: 4,
                remainder: 2,
            },
            BusRemainder {
                id: 5,
                remainder: 1,
            },
        ];
        assert_eq!(26, find_timestamp(&bus_remainders).unwrap());
    }

    #[test]
    fn solve() {
        let input = load_input("tests/day13/sample1").unwrap();
        assert_eq!(1068781, solve_buses(input).unwrap());
        for &(line, want) in &[
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486),
        ] {
            assert_eq!(want, solve_buses(parse_line(line)).unwrap())
        }
    }
}
