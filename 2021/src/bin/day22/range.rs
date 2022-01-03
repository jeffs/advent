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
    let max: i32 = max.parse()?;
    Ok(min.parse()?..(max + 1))
}

/// Returns an integer summarizing the relation between new ranges.
///
///     return   new vs old:
///     value   start     end
///     0         -        -   (no overlap)
///     1         <        <
///     2         <        =
///     3         <        >
///     4         =        <
///     5         =        =
///     6         =        >
///     7         >        <
///     8         >        =
///     9         >        >
///
/// An enum would arguably be the most Rustic way to represent these ten
/// distinct possibilities; but upon experimentation, I found the ergonomics of
/// single-digit integers compellingly better.
pub fn relation(old: &Range<i32>, new: &Range<i32>) -> usize {
    if old.start >= new.end || old.end <= new.start {
        0 // no overlap
    } else {
        let (s0, e0, s1, e1) = (old.start, old.end, new.start, new.end);
        let m = 1 + (s1 - s0).signum();
        let n = 1 + (e1 - e0).signum();
        (m * 3 + n + 1) as usize
    }
}
