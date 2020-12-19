use crate::error::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    Plus,  // +
    Times, // *
    Open,  // (
    Close, // )
    Value(u64),
}

impl Token {
    pub fn parse_all(expr: &str) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        let mut words: Vec<&str> = expr.split_whitespace().collect();
        words.reverse();
        while let Some(word) = words.pop() {
            if word.len() > 1 && !word.chars().nth(0).unwrap().is_numeric() {
                let (first, second) = word.split_at(1);
                words.push(second);
                words.push(first);
            } else if word.len() > 1 && !word.chars().nth(word.len() - 1).unwrap().is_numeric() {
                let (first, second) = word.split_at(word.len() - 1);
                words.push(second);
                words.push(first);
            } else {
                tokens.push(word.parse()?);
            }
        }
        Ok(tokens)
    }
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Token, Self::Err> {
        Ok(match s {
            "+" => Token::Plus,
            "*" => Token::Times,
            "(" => Token::Open,
            ")" => Token::Close,
            _ => Token::Value(s.parse()?),
        })
    }
}
