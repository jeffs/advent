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
        match instruction {
            Instruction::Assign(address, value) => {
                let masked = self.mask.apply(value);
                self.memory[address] = masked;
            }
            Instruction::Mask(mask) => self.mask = mask,
        }
    }

    pub fn sum(&self) -> usize {
        self.memory.sum()
    }
}
