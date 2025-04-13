use super::{Instruction, Opcode, Operand};
use crate::{cpu::Cpu, reg};
use thiserror::Error;

macro_rules! unpack_operands {
    ($tuple:ident, $type1:ident) => {
        match $tuple.0 {
            Operand::$type1(a) => a,
            _ => {
                return Err(InstructionExecuteError::UnexpectedOperand {
                    expected: stringify!($type1),
                    received: stringify!($tuple.0),
                })
            }
        }
    };
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        let operands = &self.operands;
        match self.opcode {
            Opcode::Nop => Ok(()),
            Opcode::LdBImm8 => Ok(cpu.set_register8(reg!(B), unpack_operands!(operands, Imm8))),
            Opcode::LdCImm8 => Ok(cpu.set_register8(reg!(C), unpack_operands!(operands, Imm8))),
            Opcode::LdDImm8 => Ok(cpu.set_register8(reg!(D), unpack_operands!(operands, Imm8))),
            Opcode::LdEImm8 => Ok(cpu.set_register8(reg!(E), unpack_operands!(operands, Imm8))),
            Opcode::LdHImm8 => Ok(cpu.set_register8(reg!(H), unpack_operands!(operands, Imm8))),
            Opcode::LdLImm8 => Ok(cpu.set_register8(reg!(L), unpack_operands!(operands, Imm8))),
            Opcode::LdAImm8 => Ok(cpu.set_register8(reg!(A), unpack_operands!(operands, Imm8))),
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

    #[error("Wrong operand type. Expected {expected}, received {received}")]
    UnexpectedOperand {
        expected: &'static str,
        received: &'static str,
    },
}
