use advent2020::error::{NoSolution, ParseError};
use std::fs;

const DIVISOR: u64 = 20201227;

fn transform(size: u64, subject: u64) -> u64 {
    let mut value = 1;
    for _ in 0..size {
        value *= subject;
        value %= DIVISOR;
    }
    value
}

fn find(key: u64, subject: u64) -> Result<u64, NoSolution> {
    for size in 0..DIVISOR {
        if key == transform(size, subject) {
            return Ok(size);
        }
    }
    Err(NoSolution)
}

fn parse(text: &str) -> Result<(u64, u64), ParseError> {
    let mut keys = text.split_whitespace();
    let card = keys
        .next()
        .ok_or_else(|| ParseError::new("expected card's public key"))?
        .parse()?;
    let door = keys
        .next()
        .ok_or_else(|| ParseError::new("expected door's public key"))?
        .parse()?;
    if keys.next().is_none() {
        Ok((card, door))
    } else {
        Err(ParseError::new("expected only two keys"))
    }
}

fn main() {
    let text = fs::read_to_string("tests/day25/input").unwrap();
    let (card_key, door_key) = parse(&text).unwrap();
    let card_size = find(card_key, 7).unwrap();
    let enc = transform(card_size, door_key);
    println!("{}", enc);
}

#[cfg(test)]
mod test {
    use super::*;

    const CARD_KEY: u64 = 5764801;
    const DOOR_KEY: u64 = 17807724;
    const CARD_SIZE: u64 = 8;
    const DOOR_SIZE: u64 = 11;
    const ENC_KEY: u64 = 14897079;

    #[test]
    fn transform_card() {
        assert_eq!(CARD_KEY, transform(CARD_SIZE, 7));
    }

    #[test]
    fn transform_door() {
        assert_eq!(DOOR_KEY, transform(DOOR_SIZE, 7));
    }

    #[test]
    fn find_card() {
        assert_eq!(Ok(CARD_SIZE), find(CARD_KEY, 7));
    }

    #[test]
    fn find_door() {
        assert_eq!(Ok(DOOR_SIZE), find(DOOR_KEY, 7));
    }

    #[test]
    fn enc_card() {
        assert_eq!(ENC_KEY, transform(CARD_SIZE, DOOR_KEY));
    }

    #[test]
    fn enc_door() {
        assert_eq!(ENC_KEY, transform(DOOR_SIZE, CARD_KEY));
    }
}
