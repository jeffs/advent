use crate::error::ParseError;

#[derive(Debug)]
pub enum Partial {
    Plus(u64),
    Times(u64),
    Value(u64),
}

impl Partial {
    pub fn apply(&self, v: u64) -> Result<Partial, ParseError> {
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
