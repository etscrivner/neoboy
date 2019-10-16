use super::*;
use super::registers::*;

/// State information for the Gameboy CPU.
pub struct Cpu {
    pub r: Registers,
    pub pc: Address,
    pub sp: Address
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            r: Registers::new(),
            pc: 0x0100,
            sp: 0xFFFE
        }
    }
}
