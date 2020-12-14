#![allow(dead_code, unused_variables)]
use super::address::Address;
use super::value::Value;
use std::collections::HashMap;

pub struct Sparse {
    cells: HashMap<Address, Value>,
}

impl Sparse {
    pub fn new() -> Sparse {
        Sparse {
            cells: HashMap::new(),
        }
    }

    pub fn sum(&self) -> usize {
        self.cells.values().map(|&v| usize::from(v)).sum()
    }
}
