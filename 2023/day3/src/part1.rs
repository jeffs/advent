use crate::span::{adjacencies, parse, spans};

fn is_symbol(c: u8) -> bool {
    c != b'.' && !c.is_ascii_digit()
}

pub fn solve(text: &str) -> u32 {
    let rows: Vec<Vec<u8>> = text.lines().map(|line| line.bytes().collect()).collect();

    // For each row, for each number, compute the number's column index range.
    let mut spans = spans(&rows);

    // Discard spans that don't have adjacent symbols in any direction.
    for (i, row_spans) in spans.iter_mut().enumerate() {
        row_spans.retain(|span| adjacencies(&rows, i, span).any(|(i, j)| is_symbol(rows[i][j])));
    }

    // Parse the remaining spans into numbers, and sum them.
    rows.into_iter()
        .zip(spans.into_iter())
        .flat_map(|(row, spans)| spans.into_iter().map(move |span| parse(&row, span)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 4361)
    }
}
