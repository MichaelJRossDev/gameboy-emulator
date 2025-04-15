use super::{Instruction, Opcode, OpcodeDecodeError, Operand};
use crate::cpu::Cpu;
use thiserror::Error;

impl Instruction {
    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            // Nop
            Opcode::Nop => Ok(Self {
                opcode,
                operands: (Operand::None, Operand::None),
            }),

            // Jump
            Opcode::JpImm16 => Ok(Self {
                opcode,
                operands: (Operand::Imm16(cpu.fetch_word()), Operand::None),
            }),

            // Ld r8 Imm8
            Opcode::LdBImm8
            | Opcode::LdCImm8
            | Opcode::LdDImm8
            | Opcode::LdEImm8
            | Opcode::LdHImm8
            | Opcode::LdLImm8
            | Opcode::LdAImm8 => Ok(Self {
                opcode,
                operands: (Operand::Imm8(cpu.fetch_byte()), Operand::None),
            }),

            // Ld r8 r8
            Opcode::LdBC
            | Opcode::LdBD
            | Opcode::LdBE
            | Opcode::LdBH
            | Opcode::LdBL
            | Opcode::LdBA
            | Opcode::LdCB
            | Opcode::LdCD
            | Opcode::LdCE
            | Opcode::LdCH
            | Opcode::LdCL
            | Opcode::LdCA
            | Opcode::LdDB
            | Opcode::LdDC
            | Opcode::LdDE
            | Opcode::LdDH
            | Opcode::LdDL
            | Opcode::LdDA
            | Opcode::LdEB
            | Opcode::LdEC
            | Opcode::LdED
            | Opcode::LdEH
            | Opcode::LdEL
            | Opcode::LdEA
            | Opcode::LdHB
            | Opcode::LdHC
            | Opcode::LdHD
            | Opcode::LdHE
            | Opcode::LdHL
            | Opcode::LdHA
            | Opcode::LdLB
            | Opcode::LdLC
            | Opcode::LdLD
            | Opcode::LdLE
            | Opcode::LdLH
            | Opcode::LdLA
            | Opcode::LdAB
            | Opcode::LdAC
            | Opcode::LdAD
            | Opcode::LdAE
            | Opcode::LdAH
            | Opcode::LdAL => Ok(Self {
                opcode,
                operands: (Operand::None, Operand::None),
            }),

            Opcode::LdAA
            | Opcode::LdBB
            | Opcode::LdCC
            | Opcode::LdDD
            | Opcode::LdEE
            | Opcode::LdHH
            | Opcode::LdLL => Ok(Self {opcode, operands: (Operand::None, Operand::None)}),

            Opcode::LdAFromAddr
            | Opcode::LdAddrA => Ok(Self {opcode, operands: (Operand::Address(cpu.fetch_word()), Operand::None)}),

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
