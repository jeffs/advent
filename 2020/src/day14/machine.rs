#![allow(dead_code, unused_imports, unused_variables)]

use super::instruction::Instruction;
use super::mask::Mask;
use super::memory::Sparse;

pub struct Machine {
    mask: Mask,
    memory: Sparse,
}

impl Machine {
    pub fn new(mask: Mask) -> Machine {
        Machine {
            mask,
            memory: Sparse::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        self.memory[instruction.address] = instruction.value;
    }

    pub fn sum(&self) -> usize {
        self.memory.sum()
    }
}
