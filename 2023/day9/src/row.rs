pub struct Row(Vec<i32>);

fn all_zeroes(values: &[i32]) -> bool {
    values.iter().all(|&value| value == 0)
}

/// Returns the differences between consecutive values, and the last value.
fn into_deltas(mut values: Vec<i32>) -> (Vec<i32>, Option<i32>) {
    for index in 1..values.len() {
        values[index - 1] = values[index] - values[index - 1];
    }
    let last = values.pop();
    (values, last)
}

impl Row {
    pub fn from_line(line: &str) -> Row {
        Row(line
            .split(' ')
            .map(|s| s.parse().expect("number"))
            .collect())
    }

    pub fn backward(mut self) -> Row {
        self.0.reverse();
        self
    }

    pub fn solve(mut self) -> i32 {
        let mut sum = 0;
        while !all_zeroes(&self.0) {
            let last;
            (self.0, last) = into_deltas(self.0);
            sum += last.expect("loop to have ended if vec was empty");
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("sample.txt");

    #[test]
    fn sample() {
        for (line, want) in SAMPLE.lines().zip([18, 28, 68]) {
            assert_eq!(Row::from_line(line).solve(), want);
        }
    }

    #[test]
    fn sample_backward() {
        for (line, want) in SAMPLE.lines().zip([-3, 0, 5]) {
            assert_eq!(Row::from_line(line).backward().solve(), want);
        }
    }
}
