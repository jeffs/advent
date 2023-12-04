use std::collections::HashMap;

use crate::span::{adjacencies, parse, spans};

pub fn solve(text: &str) -> u32 {
    let rows: Vec<Vec<u8>> = text.lines().map(|line| line.bytes().collect()).collect();

    let mut star_spans: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (i, row_spans) in spans(&rows).iter().enumerate() {
        for span in row_spans {
            adjacencies(&rows, i, span)
                .filter(|&(i, j)| rows[i][j] == b'*')
                .for_each(|star| {
                    star_spans
                        .entry(star)
                        .or_default()
                        .push(parse(&rows[i], span.clone()))
                });
        }
    }

    star_spans
        .values()
        .map(|spans| match spans.as_slice() {
            [x, y] => x * y,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 467835)
    }
}
