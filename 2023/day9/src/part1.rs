fn retain_deltas(mut values: Vec<i32>) -> Vec<i32> {
    for i in 1..values.len() {
        values[i - 1] = values[i] - values[i - 1];
    }
    values.pop();
    values
}

pub fn solve_line(line: &str) -> i32 {
    let mut values: Vec<i32> = line
        .split(' ')
        .map(|s| s.parse().expect("number"))
        .collect();
    let mut lasts: Vec<i32> = vec![];
    while !values.iter().all(|&value| value == 0) {
        lasts.push(*values.last().expect("loop to end when vec is empty"));
        values = retain_deltas(values);
    }
    lasts.into_iter().sum()
}

pub fn solve(text: &str) -> i32 {
    text.lines().map(solve_line).sum()
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
