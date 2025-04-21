use super::{Instruction, Opcode, OpcodeDecodeError, Operand};
use crate::cpu::Cpu;
use thiserror::Error;

impl Instruction {
    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            // Nop
            Opcode::Nop => Ok(Self {
                opcode,
                operand: Operand::None,
            }),

            // Jump
            Opcode::JpImm16 => Ok(Self {
                opcode,
                operand: Operand::Imm16(cpu.fetch_word()),
            }),

            // Ld r8 Imm8
            Opcode::LdR8(_) => Ok(Self {
                opcode,
                operand: Operand::Imm8(cpu.fetch_byte()),
            }),

            // Ld r8 r8
            Opcode::LdR8R8 { dst, src } => Ok(Self {
                opcode,
                operand: Operand::None,
            }),

            // Inc/Dec r8/r16
            Opcode::IncR8(_) | Opcode::DecR8(_) | Opcode::IncR16(_) | Opcode::DecR16(_) => {
                Ok(Self {
                    opcode,
                    operand: Operand::None,
                })
            }

            Opcode::LdAFromAddr | Opcode::LdAddrA => Ok(Self {
                opcode,
                operand: Operand::Address(cpu.fetch_word()),
            }),

            Opcode::LdR8FromMemHL(_) => Ok(Self {
                opcode,
                operand: Operand::None,
            }),

            Opcode::LdMemHLR8(_) => Ok(Self {
                opcode,
                operand: Operand::None,
            }),

            Opcode::LdR16(_) => Ok(Self {
                opcode,
                operand: Operand::Imm16(cpu.fetch_word()),
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
