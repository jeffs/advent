use advent2021::ParseError;
use std::ops::Range;

pub fn constrain(range: &Range<i32>, max: i32) -> Range<i32> {
    range.start.max(-max)..range.end.min(max + 1)
}

pub fn parse(s: &str) -> Result<Range<i32>, ParseError> {
    if s.len() < "x=1..1".len() {
        return Err(ParseError::new("bad range: too short"));
    }
    let (min, max) = s[2..]
        .split_once("..")
        .ok_or_else(|| ParseError::new("bad range: expected '..'"))?;
    let min: i32 = min.parse()?;
    let max: i32 = max.parse()?;
    assert!(min <= max);
    Ok(min..(max + 1))
}
