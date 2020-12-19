use crate::error::ParseError;
use super::partial::Partial;
use super::token::Token;

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
        assert_eq!(71, solve("1 + 2 * 3 + 4 * 5 + 6").unwrap());
    }

    #[test]
    fn solve_sample1b() {
        assert_eq!(51, solve("1 + (2 * 3) + (4 * (5 + 6))").unwrap());
    }
}
