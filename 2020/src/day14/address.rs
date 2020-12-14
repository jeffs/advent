use crate::error::ParseError;
use std::str::FromStr;

pub const LEN: usize = 36;
const MAX: usize = (1 << LEN) - 1;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Address(usize);

impl From<usize> for Address {
    fn from(u: usize) -> Self {
        Address(u)
    }
}

impl From<Address> for usize {
    fn from(a: Address) -> Self {
        a.0
    }
}

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
