use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

const PAIRS: [(u8, u8); 4] = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')];

fn load_lines<P>(input: P) -> Result<Vec<Vec<u8>>, io::Error>
where
    P: AsRef<Path>,
{
    let mut result = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        result.push(line?.into_bytes());
    }
    Ok(result)
}

fn find_matching_opener(closer: u8) -> Option<u8> {
    PAIRS
        .into_iter()
        .find(|&(_, c)| c == closer)
        .map(|(o, _)| o)
}

fn find_matching_closer(opener: u8) -> Option<u8> {
    PAIRS
        .into_iter()
        .find(|&(o, _)| o == opener)
        .map(|(_, c)| c)
}

// To support mapping over sequences of Vec<u8> lines,
// `find_corrupt_closer` accepts references to them directly, not
// references to slices.  A fn(&[u8]) can be passed a &Vec<u8> thanks to
// Borrow, but cannot be passed to Iterator<Item = Vec<u8>>::map.  For that
// reason, we ignore Clippy's suggestion that `find_corrupt_closer` accept
// a slice rather than a Vec.
#[allow(clippy::ptr_arg)]
fn find_corrupt_closer(line: &Vec<u8>) -> Option<u8> {
    let mut openers = Vec::new();
    for &c in line {
        if let Some(o) = find_matching_opener(c) {
            if openers.pop().expect("buffer underrun") != o {
                return Some(c);
            }
        } else {
            openers.push(c);
        }
    }
    None
}

mod part1 {
    use super::*;

    fn score(closer: u8) -> u64 {
        match closer {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => panic!("unexpected closer: {}", closer),
        }
    }

    pub fn solve(lines: &[Vec<u8>]) -> u64 {
        lines
            .iter()
            .filter_map(find_corrupt_closer)
            .map(score)
            .sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let lines = load_lines("tests/day10/sample").unwrap();
            assert_eq!(26397, solve(&lines));
        }
    }
}

mod part2 {
    use super::*;

    const CLOSERS: [u8; 4] = *b")]}>";

    fn find_openers(line: &[u8]) -> Vec<u8> {
        let mut openers = Vec::new();
        for c in line {
            if CLOSERS.contains(c) {
                openers.pop();
            } else {
                openers.push(*c);
            }
        }
        openers
    }

    fn complete(openers: &[u8]) -> impl Iterator<Item = u8> + '_ {
        openers.iter().cloned().flat_map(find_matching_closer).rev()
    }

    fn score_closers<I>(closers: I) -> u64
    where
        I: IntoIterator<Item = u8>,
    {
        closers
            .into_iter()
            .flat_map(|b| CLOSERS.iter().position(|&c| b == c))
            .fold(0, |score, index| score * 5 + index as u64 + 1)
    }

    #[allow(clippy::ptr_arg)]
    fn score_line(line: &Vec<u8>) -> u64 {
        score_closers(complete(&find_openers(line)))
    }

    pub fn solve(lines: &[Vec<u8>]) -> u64 {
        let mut scores: Vec<_> = lines
            .iter()
            .filter(|v| find_corrupt_closer(v).is_none())
            .map(score_line)
            .collect();
        let n = scores.len() / 2;
        *scores.select_nth_unstable(n).1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_score_closers() {
            assert_eq!(288957, score_closers(*b"}}]])})]"));
        }

        #[test]
        fn test_solve() {
            let lines = load_lines("tests/day10/sample").unwrap();
            assert_eq!(288957, solve(&lines));
        }
    }
}

fn main() {
    let input = "tests/day10/input";
    let lines = load_lines(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&lines));
    println!("{}", part2::solve(&lines));
}
