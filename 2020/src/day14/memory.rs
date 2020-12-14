use super::address::Address;
use super::value::Value;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct Sparse {
    default: Value,
    values: HashMap<Address, Value>,
}

impl Sparse {
    pub fn new() -> Sparse {
        Sparse {
            default: Value::default(),
            values: HashMap::new(),
        }
    }

    pub fn sum(&self) -> usize {
        self.values.values().map(|&v| usize::from(v)).sum()
    }
}

impl Index<Address> for Sparse {
    type Output = Value;

    fn index(&self, address: Address) -> &Self::Output {
        match self.values.get(&address) {
            Some(value) => value,
            None => &self.default,
        }
    }
}

impl IndexMut<Address> for Sparse {
    fn index_mut(&mut self, address: Address) -> &mut Self::Output {
        if !self.values.contains_key(&address) {
            self.values.insert(address, self.default);
        }
        self.values.get_mut(&address).unwrap()
    }
}
