pub fn solve_line(_text: &str) -> i32 {
    todo!()
}

pub fn solve(_text: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn solve_line_sample() {
        for (line, want) in SAMPLE.lines().zip([18, 28, 68]) {
            assert_eq!(solve_line(line), want);
        }
    }
    #[test]
    fn sample() {
        assert_eq!(solve(SAMPLE), 114);
    }
}
