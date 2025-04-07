use thiserror::Error;

use crate::{
    instruction::{
        Instruction, InstructionDecodeError, InstructionExecuteError, Opcode, OpcodeDecodeError,
    },
    memory::MemoryBus,
};

pub struct Cpu {
    pc: u16,
    memory: Box<dyn MemoryBus>,
}

#[derive(Debug, Error)]
pub enum CpuStepError {
    #[error("Failed to decode opcode: {0}")]
    Opcode(#[from] OpcodeDecodeError),

    #[error("Failed to decode instruction: {0}")]
    Decode(#[from] InstructionDecodeError),

    #[error("Failed to execute instruction: {0}")]
    Execute(#[from] InstructionExecuteError),
}

impl Cpu {
    pub fn fetch_byte(&mut self) -> u8 {
        let value = self.memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    pub fn new(memory: Box<dyn MemoryBus>) -> Self {
        Self { memory, pc: 0 }
    }

    fn step(&mut self) -> Result<(), CpuStepError> {
        let opcode = Opcode::try_from(self.fetch_byte())?;
        let instruction = Instruction::decode(opcode, self)?;
        instruction.execute(self)?;
        Ok(())
    }
}
