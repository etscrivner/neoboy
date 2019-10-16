use super::*;
use super::cpu::*;
use super::memory::*;
use super::operations::*;

pub struct Machine {
    pub cpu: Cpu,
    pub memory: Memory
}

impl Machine {
    pub fn new(memory: Memory) -> Self {
        Self { cpu: Cpu::new(), memory: memory }
    }

    pub fn step(&mut self) -> GameboyResult<Cycles> {
        let result = Operation::from_memory(self.cpu.pc, &self.memory);

        if let Ok(operation) = result {
            match operation.opcode {
                Opcode::Nop => { self.cpu.pc += 1; Ok(1) },
                _ => {
                    return Err(GameboyError::new(
                        GameboyErrorKind::Unknown(format!("Unimplemented: {:?}", operation.opcode))
                    ));
                }
            }
        } else {
            Err(result.err().unwrap())
        }
    }
}
