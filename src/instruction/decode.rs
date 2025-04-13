use super::{Instruction, Opcode, OpcodeDecodeError, Operand};
use crate::cpu::Cpu;
use thiserror::Error;

impl Instruction {
    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            Opcode::Nop => Ok(Self {
                opcode,
                operands: (Operand::None, Operand::None),
            }),
            Opcode::LdBImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdCImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdDImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdEImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdHImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdLImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),
            Opcode::LdAImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
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
