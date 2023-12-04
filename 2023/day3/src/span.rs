use std::ops::Range;

/// Returns all (row, column) coordinates adjacent to rows[i][*span].
pub fn adjacencies(
    rows: &[Vec<u8>],
    i: usize,
    span: &Range<usize>,
) -> impl Iterator<Item = (usize, usize)> {
    let mut adjacenies = Vec::new();
    if i > 0 {
        if span.start > 0 {
            // Northwest.
            adjacenies.push((i - 1, span.start - 1));
        }
        // North.
        adjacenies.extend(span.clone().map(|j| (i - 1, j)));
        if span.end < rows[i - 1].len() {
            // Northeast.
            adjacenies.push((i - 1, span.end))
        }
    }
    if span.start > 0 {
        // West.
        adjacenies.push((i, span.start - 1));
    }
    if span.end < rows[i].len() {
        // East.
        adjacenies.push((i, span.end));
    }
    if i + 1 < rows.len() {
        if span.start > 0 {
            // Southwest.
            adjacenies.push((i + 1, span.start - 1));
        }
        // South.
        adjacenies.extend(span.clone().map(|j| (i + 1, j)));
        if span.end < rows[i + 1].len() {
            // Southeast.
            adjacenies.push((i + 1, span.end));
        }
    }

    adjacenies.into_iter()
}

pub fn parse(row: &[u8], span: Range<usize>) -> u32 {
    std::str::from_utf8(&row[span])
        .expect("number spans are UTF-8")
        .parse::<u32>()
        .expect("number spans are digits")
}

/// Returns index ranges of all digit sequences in each row.
pub fn spans(rows: &[Vec<u8>]) -> Vec<Vec<Range<usize>>> {
    rows.iter()
        .map(|row| {
            let mut row_spans = Vec::new();
            let mut start = None;
            for (i, c) in row.iter().enumerate() {
                if c.is_ascii_digit() {
                    if start.is_none() {
                        start = Some(i);
                    }
                } else if let Some(start) = start.take() {
                    row_spans.push(start..i);
                }
            }
            if let Some(start) = start {
                row_spans.push(start..row.len());
            }
            row_spans
        })
        .collect()
}
