#![allow(dead_code, unused_imports, unused_variables)]
use advent2020::error::{NoSolution, ParseError};
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Token {
    Plus,  // +
    Times, // *
    Open,  // (
    Close, // )
    Value(u64),
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

#[derive(Debug)]
enum Partial {
    Plus(u64),
    Times(u64),
    Value(u64),
}

impl Partial {
    fn apply(&self, v: u64) -> Result<Partial, ParseError> {
        Ok(Partial::Value(match self {
            Partial::Plus(u) => u + v,
            Partial::Times(u) => u * v,
            Partial::Value(u) => {
                let what = format!("consecutive values: {} {}", u, v);
                return Err(ParseError::new(what));
            }
        }))
    }
}

fn eval_imp(tokens: &mut Vec<Token>) -> Result<u64, ParseError> {
    let mut op: Option<Partial> = None;
    while let Some(token) = tokens.pop() {
        op = match token {
            Token::Plus => Some(Partial::Plus(match op {
                Some(Partial::Value(u)) => u,
                _ => return Err(ParseError::new("unexpected +")),
            })),
            Token::Times => Some(Partial::Times(match op {
                Some(Partial::Value(u)) => u,
                _ => return Err(ParseError::new("unexpected *")),
            })),
            Token::Open => {
                let v = eval_imp(tokens)?;
                match op {
                    Some(part) => Some(part.apply(v)?),
                    None => Some(Partial::Value(v)),
                }
            }
            Token::Close => break,
            Token::Value(v) => match op {
                Some(part) => Some(part.apply(v)?),
                None => Some(Partial::Value(v)),
            },
        };
    }
    match op {
        Some(Partial::Plus(_)) => Err(ParseError::new("unsatisified +")),
        Some(Partial::Times(_)) => Err(ParseError::new("unsatisified *")),
        Some(Partial::Value(u)) => Ok(u),
        None => Err(ParseError::new("expected tokens")),
    }
}

fn eval(mut tokens: Vec<Token>) -> Result<u64, ParseError> {
    tokens.reverse();
    eval_imp(&mut tokens)
}

fn parse_tokens(expr: &str) -> Result<Vec<Token>, ParseError> {
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

fn solve_part1(text: &str) -> Result<u64, ParseError> {
    let mut sum = 0;
    for line in text.lines() {
        sum += eval(parse_tokens(line)?)?;
    }
    Ok(sum)
}

fn main() {
    let input = "tests/day18/input";
    let text = match fs::read_to_string(input) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        }
    };
    match solve_part1(&text) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_sample1a() {
        // no parens
        assert_eq!(71, solve_part1("1 + 2 * 3 + 4 * 5 + 6").unwrap());
    }

    #[test]
    fn solve_part1_sample1b() {
        assert_eq!(51, solve_part1("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
    }
}
