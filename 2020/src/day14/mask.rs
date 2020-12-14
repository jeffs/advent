use super::address::Address;
use super::masked::Masked;
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

    pub fn new() -> Mask {
        Mask { clear: 0, set: 0 }
    }

    pub fn address(&self, address: Address) -> Masked {
        // A bit is floating if the mask neither clears nor sets it.
        // The first address to yield has all floating bits zeroed.
        // Note that 0 in an address mask leaves bits unchanged.
        let float = !(self.clear | self.set) & ((1 << LEN) - 1);
        let first = Address::from((usize::from(address) | self.set) & !float);
        Masked::new(float, first)
    }

    pub fn value(&self, value: Value) -> Value {
        Value::from(usize::from(value) & !self.clear | self.set)
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
    fn value() {
        let value = Value::from(11);
        let mask = Mask::parse("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(73, usize::from(mask.value(value)));
    }

    #[test]
    fn address() {
        let mask = Mask::parse("000000000000000000000000000000X1001X").unwrap();
        let mut addresses = mask.address(Address::from(42));
        assert_eq!(26, usize::from(addresses.next().unwrap()))
    }
}
