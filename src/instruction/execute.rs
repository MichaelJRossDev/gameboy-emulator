use super::{Instruction, Opcode, Operand};
use crate::{cpu::{registers::Register8, Cpu}, reg};
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

macro_rules! ld_r8_r8 {
    ($cpu:ident, $set:ident, $get:ident) => {
        $cpu.set_register8(Register8::$set, $cpu.get_register8(Register8::$get))
    };
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        let operands = &self.operands;
        match self.opcode {
            // Nop
            Opcode::Nop => Ok(()),

            // Jump
            Opcode::JpImm16 => Ok(cpu.set_pc(unpack_operands!(operands, Imm16))),

            // Ld r8 imm8
            Opcode::LdBImm8 => Ok(cpu.set_register8(reg!(B), unpack_operands!(operands, Imm8))),
            Opcode::LdCImm8 => Ok(cpu.set_register8(reg!(C), unpack_operands!(operands, Imm8))),
            Opcode::LdDImm8 => Ok(cpu.set_register8(reg!(D), unpack_operands!(operands, Imm8))),
            Opcode::LdEImm8 => Ok(cpu.set_register8(reg!(E), unpack_operands!(operands, Imm8))),
            Opcode::LdHImm8 => Ok(cpu.set_register8(reg!(H), unpack_operands!(operands, Imm8))),
            Opcode::LdLImm8 => Ok(cpu.set_register8(reg!(L), unpack_operands!(operands, Imm8))),
            Opcode::LdAImm8 => Ok(cpu.set_register8(reg!(A), unpack_operands!(operands, Imm8))),

            // Ld r8 r8
            Opcode::LdBC => Ok(ld_r8_r8!(cpu, B, C)),
            Opcode::LdBD => Ok(ld_r8_r8!(cpu, B, D)),
            Opcode::LdBE => Ok(ld_r8_r8!(cpu, B, E)),
            Opcode::LdBH => Ok(ld_r8_r8!(cpu, B, H)),
            Opcode::LdBL => Ok(ld_r8_r8!(cpu, B, L)),
            Opcode::LdBA => Ok(ld_r8_r8!(cpu, B, A)),
            Opcode::LdCB => Ok(ld_r8_r8!(cpu, C, B)),
            Opcode::LdCD => Ok(ld_r8_r8!(cpu, C, D)),
            Opcode::LdCE => Ok(ld_r8_r8!(cpu, C, E)),
            Opcode::LdCH => Ok(ld_r8_r8!(cpu, C, H)),
            Opcode::LdCL => Ok(ld_r8_r8!(cpu, C, L)),
            Opcode::LdCA => Ok(ld_r8_r8!(cpu, C, A)),
            Opcode::LdDB => Ok(ld_r8_r8!(cpu, D, B)),
            Opcode::LdDC => Ok(ld_r8_r8!(cpu, D, C)),
            Opcode::LdDE => Ok(ld_r8_r8!(cpu, D, E)),
            Opcode::LdDH => Ok(ld_r8_r8!(cpu, D, H)),
            Opcode::LdDL => Ok(ld_r8_r8!(cpu, D, L)),
            Opcode::LdDA => Ok(ld_r8_r8!(cpu, D, A)),
            Opcode::LdEB => Ok(ld_r8_r8!(cpu, E, B)),
            Opcode::LdEC => Ok(ld_r8_r8!(cpu, E, C)),
            Opcode::LdED => Ok(ld_r8_r8!(cpu, E, D)),
            Opcode::LdEH => Ok(ld_r8_r8!(cpu, E, H)),
            Opcode::LdEL => Ok(ld_r8_r8!(cpu, E, L)),
            Opcode::LdEA => Ok(ld_r8_r8!(cpu, E, A)),
            Opcode::LdHB => Ok(ld_r8_r8!(cpu, H, B)),
            Opcode::LdHC => Ok(ld_r8_r8!(cpu, H, C)),
            Opcode::LdHD => Ok(ld_r8_r8!(cpu, H, D)),
            Opcode::LdHE => Ok(ld_r8_r8!(cpu, H, E)),
            Opcode::LdHL => Ok(ld_r8_r8!(cpu, H, L)),
            Opcode::LdHA => Ok(ld_r8_r8!(cpu, H, A)),
            Opcode::LdLB => Ok(ld_r8_r8!(cpu, L, B)),
            Opcode::LdLC => Ok(ld_r8_r8!(cpu, L, C)),
            Opcode::LdLD => Ok(ld_r8_r8!(cpu, L, D)),
            Opcode::LdLE => Ok(ld_r8_r8!(cpu, L, E)),
            Opcode::LdLH => Ok(ld_r8_r8!(cpu, L, H)),
            Opcode::LdLA => Ok(ld_r8_r8!(cpu, L, A)),
            Opcode::LdAB => Ok(ld_r8_r8!(cpu, A, B)),
            Opcode::LdAC => Ok(ld_r8_r8!(cpu, A, C)),
            Opcode::LdAD => Ok(ld_r8_r8!(cpu, A, D)),
            Opcode::LdAE => Ok(ld_r8_r8!(cpu, A, E)),
            Opcode::LdAH => Ok(ld_r8_r8!(cpu, A, H)),
            Opcode::LdAL => Ok(ld_r8_r8!(cpu, A, L)),

            // Redundant Ld r8 r8
            Opcode::LdAA
            | Opcode::LdBB
            | Opcode::LdCC
            | Opcode::LdDD
            | Opcode::LdEE
            | Opcode::LdHH
            | Opcode::LdLL => Ok(()),

            // Ld MemHL r8
            Opcode::LdMemHLA => todo!(),
            Opcode::LdMemHLB => todo!(),
            Opcode::LdMemHLC => todo!(),
            Opcode::LdMemHLD => todo!(),
            Opcode::LdMemHLE => todo!(),
            Opcode::LdMemHLH => todo!(),
            Opcode::LdMemHLL => todo!(),

            // Ld r8 MemHL
            Opcode::LdAFromMemHL => todo!(),
            Opcode::LdBFromMemHL => todo!(),
            Opcode::LdCFromMemHL => todo!(),
            Opcode::LdDFromMemHL => todo!(),
            Opcode::LdEFromMemHL => todo!(),
            Opcode::LdHFromMemHL => todo!(),
            Opcode::LdLFromMemHL => todo!(),

            // Ld A from Address
            Opcode::LdAFromAddr => Ok(cpu.set_register8(reg!(A), cpu.read(unpack_operands!(operands, Address)))),
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
