use super::instruction::Instruction;
use super::mask::Mask;
use super::memory::Sparse;

pub struct Machine {
    mask: Mask,
    memory: Sparse,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            mask: Mask::new(),
            memory: Sparse::new(),
        }
    }

    pub fn execute1(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Assign(address, value) => {
                let masked = self.mask.value(value);
                self.memory[address] = masked;
            }
            Instruction::Mask(mask) => self.mask = mask,
        }
    }

    pub fn execute2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Assign(address, value) => {
                for masked in self.mask.address(address) {
                    self.memory[masked] = value;
                }
            }
            Instruction::Mask(mask) => self.mask = mask,
        }
    }

    pub fn sum(&self) -> usize {
        self.memory.sum()
    }
}
