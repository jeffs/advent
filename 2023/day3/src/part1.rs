#[derive(Debug)]
struct Span {
    begin: usize,
    end: usize,
}

impl Span {
    fn chars<'a>(&self, row: &'a str) -> impl Iterator<Item = char> + 'a {
        let span = (self.begin.max(1) - 1)..(self.end.min(row.len() - 1) + 1);
        row.chars().take(span.end).skip(span.start)
    }
}

struct Spans<'a> {
    orig_len: usize,
    rest: &'a str,
}

impl Spans<'_> {
    fn from(orig: &str) -> Spans {
        Spans {
            orig_len: orig.len(),
            rest: orig,
        }
    }
}

impl Iterator for Spans<'_> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        self.rest = self.rest.trim_start_matches(|c: char| !c.is_numeric());
        let begin = self.orig_len - self.rest.len();
        self.rest = self.rest.trim_start_matches(char::is_numeric);
        let end = self.orig_len - self.rest.len();
        (begin != end).then_some(Span { begin, end })
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn matched_value(rows: &[&str], i: usize, m: &Span) -> Option<u32> {
    // When checking whether adjacent row indexes are in range, we use 1-based
    // indexes to avoid computing k=i-1, since that would cause underflow from
    // the top row (when i=0).
    let has_symbol = |&s| m.chars(s).any(is_symbol);
    (0..3)
        .any(|di| {
            let k = i + di; // Index of row to check for symbol, plus one.
            (1..rows.len() + 1).contains(&k) && has_symbol(&rows[k - 1])
        })
        .then(|| {
            rows[i][m.begin..m.end]
                .parse::<u32>()
                .expect("matches are base 10 numbers")
        })
}

/// Returns the values of all matches in rows[i] having adjacent symbols.
fn matched_values<'a>(rows: &'a [&'a str], i: usize) -> impl Iterator<Item = u32> + 'a {
    Spans::from(rows[i]).filter_map(move |m| matched_value(rows, i, &m))
}

pub fn solve(text: &str) -> u32 {
    let rows: Vec<&str> = text.lines().collect();
    (0..rows.len()).flat_map(|i| matched_values(&rows, i)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 4361)
    }
}
