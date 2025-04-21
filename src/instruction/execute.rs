use super::{Instruction, Opcode, Operand};
use crate::{
    cpu::{Cpu, registers::Register8},
    reg, unpack_operand,
};
use thiserror::Error;

macro_rules! ld_r8_r8 {
    ($cpu:ident, $set:ident, $get:ident) => {
        $cpu.set_register8(Register8::$set, $cpu.get_register8(Register8::$get))
    };
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        let operand = &self.operand;
        match self.opcode {
            // Nop
            Opcode::Nop => Ok(()),

            // Jump
            Opcode::JpImm16 => Ok(cpu.set_pc(unpack_operand!(operand, Imm16))),

            // Ld r8 imm8
            Opcode::LdR8(r8) => Ok(cpu.set_register8(r8, unpack_operand!(operand, Imm8))),

            // Ld r8 r8
            Opcode::LdR8R8 { dst, src } => Ok(cpu.set_register8(dst, cpu.get_register8(src))),

            // Ld MemHL r8
            Opcode::LdMemHLA => todo!(),
            Opcode::LdMemHLB => todo!(),
            Opcode::LdMemHLC => todo!(),
            Opcode::LdMemHLD => todo!(),
            Opcode::LdMemHLE => todo!(),
            Opcode::LdMemHLH => todo!(),
            Opcode::LdMemHLL => todo!(),

            // Ld r8 MemHL
            Opcode::LdR8FromMemHL(reg) => {
                Ok(cpu.set_register8(reg, cpu.read(cpu.get_register16(reg!(HL)))))
            }

            // Ld A from Address
            Opcode::LdAFromAddr => {
                Ok(cpu.set_register8(reg!(A), cpu.read(unpack_operand!(operand, Address))))
            }

            Opcode::LdAddrA => Ok(cpu.write(
                unpack_operand!(operand, Address),
                cpu.get_register8(reg!(A)),
            )),

            // Inc r8
            Opcode::IncR8(reg) => Ok(cpu.add_r8(reg, 1)),

            // Dec r8
            Opcode::DecR8(reg) => Ok(cpu.sub_r8(reg, 1)),

            // Inc r16
            Opcode::IncR16(reg) => Ok(cpu.add_r16(reg, 1)),

            // Dec r16
            Opcode::DecR16(reg) => Ok(cpu.sub_r16(reg, 1)),
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
