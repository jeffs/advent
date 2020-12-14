use super::address::{Address, LEN};
use std::mem;

fn find_next(float: usize, address: Address) -> Option<Address> {
    let mut address = usize::from(address);
    for i in 0..LEN {
        let m = 1 << i;
        if float & m == 0 {
            // Skip past bits that aren't floating.
        } else if address & m == 0 {
            // Flip the lowest floating 0 bit to a 1.
            return Some(Address::from(address | m));
        } else {
            // Clear the floating 1 bit.
            address &= !m;
        }
    }
    None // We didn't find any floating 0 bits, so we're done here.
}

/// Iterator over masked address values.  Each floating bit in the source
/// mask doubles the number of results a Masked will yield.
#[derive(Debug)]
pub struct Masked {
    float: usize, // bitmask for floating bits of the address
    next: Option<Address>,
}

impl Masked {
    pub fn new(float: usize, first: Address) -> Masked {
        Masked {
            float,
            next: Some(first),
        }
    }
}

impl Iterator for Masked {
    type Item = Address;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(address) = self.next {
            mem::replace(&mut self.next, find_next(self.float, address))
        } else {
            None
        }
    }
}
