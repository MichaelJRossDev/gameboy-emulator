use thiserror::Error;
use super::{Instruction, Opcode, OpcodeDecodeError, Operand};
use crate::cpu::Cpu;

impl Instruction {
    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            Opcode::Nop => Ok(Self {
                opcode: Opcode::Nop,
                operands: (Operand::None, Operand::None),
            }),
            _ => Err(InstructionDecodeError::NotImplemented),
        }
    }
}

#[derive(Debug, Error)]
pub enum InstructionDecodeError {
    #[error("Opcode decoding failed: {0}")]
    Opcode(#[from] OpcodeDecodeError),

    #[error("Invalid operands or malformed instruction")]
    OperandFormat,

    #[error("Instruction not yet implemented")]
    NotImplemented,
}
