#[derive(Debug)]
struct Match {
    begin: usize,
    end: usize,
}

impl Match {
    fn chars<'a>(&self, row: &'a str) -> impl Iterator<Item = char> + 'a {
        let span = (self.begin.max(1) - 1)..(self.end.min(row.len() - 1) + 1);
        row.chars().take(span.end).skip(span.start)
    }
}

struct Matches<'a> {
    orig_len: usize,
    rest: &'a str,
}

impl Matches<'_> {
    fn from(orig: &str) -> Matches {
        Matches {
            orig_len: orig.len(),
            rest: orig,
        }
    }
}

impl Iterator for Matches<'_> {
    type Item = Match;

    fn next(&mut self) -> Option<Self::Item> {
        self.rest = self.rest.trim_start_matches(|c: char| !c.is_numeric());
        let begin = self.orig_len - self.rest.len();
        self.rest = self.rest.trim_start_matches(char::is_numeric);
        let end = self.orig_len - self.rest.len();
        (begin != end).then_some(Match { begin, end })
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

pub fn solve(text: &str) -> u32 {
    // Check each row for matches, then parse and return each match only if it
    // has an adjacent symbol.  When checking whether adjacent row indexes are
    // in range, we use 1-based indexes to avoid computing -1 for the "index"
    // above the top row, since that would cause underflow of the index type
    // (which is unsigned).
    let rows: Vec<&str> = text.lines().collect();
    rows.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            let rows = &rows;
            Matches::from(row).filter_map(move |match_| {
                let has_symbol = |&s| match_.chars(s).any(is_symbol);
                (0..3)
                    .any(|di| {
                        let k = i + di; // Index of row to check for symbol, plus one.
                        (1..rows.len() + 1).contains(&k) && has_symbol(&rows[k - 1])
                    })
                    .then(|| {
                        row[match_.begin..match_.end]
                            .parse::<u32>()
                            .expect("matches are base 10 numbers")
                    })
            })
        })
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
