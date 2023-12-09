pub struct Row(Vec<i32>);

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
        while !self.0.iter().all(|&value| value == 0) {
            for i in 1..self.0.len() {
                self.0[i - 1] = self.0[i] - self.0[i - 1];
            }
            sum += self.0.pop().expect("loop to end if vec is empty");
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
