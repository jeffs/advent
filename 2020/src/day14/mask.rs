use super::value::Value;
use crate::error::ParseError;

const LEN: usize = 36;

fn slice_text(line: &str) -> Result<&str, ParseError> {
    let mut parts = line.splitn(3, ' ');
    match (parts.next(), parts.next(), parts.next()) {
        (Some("mask"), Some("="), Some(text)) => Ok(text),
        _ => Err(ParseError::new(format!("{}: expected mask", line))),
    }
}

#[derive(Debug)]
pub struct Mask {
    clear: usize,
    set: usize,
}

impl Mask {
    // As of this writing, overloading the call operator is unstable.
    // See: https://doc.rust-lang.org/std/ops/trait.Fn.html#required-methods
    pub fn apply(&self, value: Value) -> Value {
        Value::from(usize::from(value) & !self.clear | self.set)
    }

    fn parse(text: &str) -> Result<Mask, ParseError> {
        let mut mask = Mask { clear: 0, set: 0 };
        for (i, b) in text.bytes().enumerate() {
            match b {
                b'X' => (),
                b'0' => mask.clear |= 1 << (LEN - i - 1),
                b'1' => mask.set |= 1 << (LEN - i - 1),
                _ => {
                    let what = format!("{}: bad mask bit", b);
                    return Err(ParseError::new(what));
                }
            };
        }
        Ok(mask)
    }

    pub fn parse_line<S: AsRef<str>>(line: S) -> Result<Mask, ParseError> {
        let text = slice_text(line.as_ref())?;
        if text.len() == LEN {
            Mask::parse(text)
        } else {
            let what = format!("{}: bad mask: expected {} bits", text, LEN);
            Err(ParseError::new(what))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply() {
            let value = Value::from(11);
            let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
            assert_eq!(73, usize::from(mask.apply(value)));
    }
}
