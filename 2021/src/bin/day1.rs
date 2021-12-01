mod day1 {
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead as _, BufReader};
    use std::path::Path;

    pub fn load_depths<P: AsRef<Path>>(input: P) -> Result<Vec<u32>, Box<dyn Error>> {
        let mut depths = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            depths.push(line?.parse()?);
        }
        Ok(depths)
    }

    pub mod part1 {
        pub fn solve(depths: &[u32]) -> usize {
            (1..depths.len())
                .filter(|&i| depths[i] > depths[i - 1])
                .count()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_depths;
            use super::solve;

            #[test]
            fn test_solve() {
                assert_eq!(7, solve(&load_depths("tests/day1/sample").unwrap()));
            }
        }
    }

    pub mod part2 {
        const WINDOW_SIZE: usize = 3;

        pub fn solve(depths: &[u32]) -> usize {
            (WINDOW_SIZE..depths.len())
                .filter(|&i| {
                    let j = i + 1;
                    let old: u32 = depths[i - WINDOW_SIZE..i].iter().sum();
                    let new: u32 = depths[j - WINDOW_SIZE..j].iter().sum();
                    new > old
                })
                .count()
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
}

fn main() {
    let input = "tests/day1/input";
    let depths = day1::load_depths(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day1::part1::solve(&depths));
    println!("{}", day1::part2::solve(&depths));
}
