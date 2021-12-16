use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn load_depths<P: AsRef<Path>>(input: P) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut depths = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        depths.push(line?.parse()?);
    }
    Ok(depths)
}

pub mod part1 {
    // O(N) time, O(1) space
    pub fn solve<I>(depths: I) -> usize
    where
        I: IntoIterator<Item = u32>,
    {
        depths
            .into_iter()
            .scan(None, |old: &mut Option<u32>, new| {
                Some(old.replace(new).filter(|&old| new > old).is_some())
            })
            .filter(|&b| b)
            .count()
    }

    #[cfg(test)]
    mod tests {
        use super::super::load_depths;
        use super::solve;

        #[test]
        fn test_solve() {
            let depths = load_depths("tests/day1/sample").unwrap();
            assert_eq!(7, solve(depths.iter().cloned()));
        }
    }
}

pub mod part2 {
    use super::part1;

    const WINDOW_SIZE: usize = 3;

    // O(N) time, O(1) space; but requires input taking O(N) space
    pub fn solve(depths: &[u32]) -> usize {
        part1::solve(depths.windows(WINDOW_SIZE).map(|win| win.iter().sum()))
    }

    #[cfg(test)]
    mod tests {
        use super::super::load_depths;
        use super::solve;

        #[test]
        fn test_solve() {
            assert_eq!(5, solve(&load_depths("tests/day1/sample").unwrap()));
        }
    }
}

fn main() {
    let input = "tests/day1/input";
    let depths = load_depths(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(depths.iter().cloned()));
    println!("{}", part2::solve(&depths));
}
