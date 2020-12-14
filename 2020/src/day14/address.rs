use crate::error::ParseError;
use std::str::FromStr;

const MAX: usize = (1 << 36) - 1;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address(usize);

impl FromStr for Address {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Address, Self::Err> {
        let address: usize = s.parse()?;
        if MAX < address {
            let what = format!("{}: address is too large", address);
            Err(ParseError::new(what))
        } else {
            Ok(Address(address))
        }
    }
}
