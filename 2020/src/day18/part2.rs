use super::partial::Partial;
use super::token::Token;
use crate::error::ParseError;

fn push_value(parts: &mut Vec<Partial>, v: u64) {
    let last = parts.pop();
    if let Some(Partial::Plus(u)) = last {
        parts.push(Partial::Value(u + v));
    } else {
        if let Some(part) = last {
            parts.push(part);
        }
        parts.push(Partial::Value(v));
    }
}

fn product(parts: &[Partial]) -> Result<u64, ParseError> {
    let values: Vec<u64> = parts
        .iter()
        .filter_map(|term| match term {
            Partial::Times(u) => Some(*u),
            Partial::Value(u) => Some(*u),
            _ => None,
        })
        .collect();
    if values.len() == parts.len() {
        Ok(values.iter().product())
    } else {
        Err(ParseError::new("bad syntax"))
    }
}

fn eval_imp(tokens: &mut Vec<Token>) -> Result<u64, ParseError> {
    let mut parts = Vec::new();
    while let Some(token) = tokens.pop() {
        match token {
            Token::Plus => {
                if let Some(Partial::Value(u)) = parts.pop() {
                    parts.push(Partial::Plus(u));
                } else {
                    return Err(ParseError::new("unexpected +"));
                }
            }
            Token::Times => {
                if let Some(Partial::Value(u)) = parts.pop() {
                    parts.push(Partial::Times(u));
                } else {
                    return Err(ParseError::new("unexpected *"));
                }
            }
            Token::Open => push_value(&mut parts, eval_imp(tokens)?),
            Token::Close => break,
            Token::Value(v) => push_value(&mut parts, v),
        };
    }
    product(&parts)
}

fn eval(mut tokens: Vec<Token>) -> Result<u64, ParseError> {
    tokens.reverse();
    eval_imp(&mut tokens)
}

pub fn solve(text: &str) -> Result<u64, ParseError> {
    let mut sum = 0;
    for line in text.lines() {
        sum += eval(Token::parse_all(line)?)?;
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1a() {
        // no parens
        assert_eq!(231, solve("1 + 2 * 3 + 4 * 5 + 6").unwrap());
    }

    #[test]
    fn solve_sample1b() {
        assert_eq!(51, solve("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
    }
}
