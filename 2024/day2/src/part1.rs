use crate::input::{Input, Report};

pub type Error = crate::input::LevelError;
pub type Result<T> = std::result::Result<T, Error>;

pub fn is_safe(Report(levels): &Report) -> bool {
    let is_increasing = levels[0] < levels[1];
    levels[0..levels.len() - 1]
        .iter()
        .zip(levels[1..levels.len()].iter())
        .all(|(&x, &y)| (x < y) == is_increasing && (1..4).contains(&x.abs_diff(y)))
}

#[must_use]
pub fn count_safe(Input(reports): &Input) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe() {
        assert_eq!(count_safe(&Input::sample()), 2);
    }
}
