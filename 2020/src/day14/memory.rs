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
}
