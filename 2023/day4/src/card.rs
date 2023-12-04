use std::collections::HashSet;

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_ascii_whitespace()
        .map(|word| word.parse().expect("number"))
        .collect()
}

pub struct Card {
    pub id: usize,
    pub count: usize, // how many winning numbers
}

impl Card {
    pub fn from_line(line: &str) -> Card {
        let (head, body) = line.split_once(':').expect("colon");
        let (want, got) = body.split_once("|").expect("pipe");
        let id = head[head.find(char::is_numeric).expect("card ID")..]
            .parse()
            .expect("card ID to be a number");
        let count = parse_numbers(want)
            .intersection(&parse_numbers(got))
            .count();
        Card { id, count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        for (index, (line, count)) in include_str!("sample.txt")
            .lines()
            .zip([4, 2, 2, 1, 0, 0])
            .enumerate()
        {
            let card = Card::from_line(line);
            assert_eq!(card.id, index + 1);
            assert_eq!(card.count, count);
        }
    }
}
