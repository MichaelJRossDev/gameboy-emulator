use thiserror::Error;
use crate::cpu::Cpu;
use super::{Opcode, Instruction};

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        match self.opcode {
            Opcode::Nop => Ok(()),
            _ => Err(InstructionExecuteError::NotImplemented),
        }
    }
}

#[derive(Debug, Error)]
pub enum InstructionExecuteError {
    #[error("Unsupported instruction execution")]
    NotImplemented,

    #[error("Invalid register access")]
    InvalidRegister,

    #[error("Memory access failed")]
    MemoryError,
}

