use crate::{
    input::{Input, Report},
    part1,
};

pub type Error = crate::input::LevelError;
pub type Result<T> = std::result::Result<T, Error>;

fn is_safe(report: &Report) -> bool {
    let remove = |index| {
        let mut report = report.clone();
        report.remove(index);
        report
    };
    part1::is_safe(report) || (0..report.len()).any(|index| part1::is_safe(&remove(index)))
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
        assert_eq!(count_safe(&Input::sample()), 4);
    }
}
