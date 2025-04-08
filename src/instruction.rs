pub mod opcode;
pub mod operand;

use crate::cpu::Cpu;
pub use opcode::{Opcode, OpcodeDecodeError};
pub use operand::Operand;
use thiserror::Error;

pub struct Instruction {
    opcode: Opcode,
    operands: (Operand, Operand),
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

#[derive(Debug, thiserror::Error)]
pub enum InstructionExecuteError {
    #[error("Unsupported instruction execution")]
    NotImplemented,

    #[error("Invalid register access")]
    InvalidRegister,

    #[error("Memory access failed")]
    MemoryError,
}

impl Instruction {
    pub fn execute(&self, cpu: &mut Cpu) -> Result<(), InstructionExecuteError> {
        match self.opcode {
            Opcode::Nop => Ok(()),
            _ => Err(InstructionExecuteError::NotImplemented),
        }
    }

    pub fn decode(opcode: Opcode, cpu: &mut Cpu) -> Result<Self, InstructionDecodeError> {
        match opcode {
            Opcode::Nop => Ok(Self::nop()),
            _ => Err(InstructionDecodeError::NotImplemented),
        }
    }

    fn nop() -> Self {
        Self {
            opcode: Opcode::Nop,
            operands: (Operand::None, Operand::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu::CpuSnapshot, memory::{self, flat_memory, FlatMemory}};

    use super::*;

    #[test]
    fn nop_only_advances_pc() {
        let mut cpu = Cpu::cpu_with_program(&[0x00]);
        let before = CpuSnapshot::from(&cpu);

        cpu.step().unwrap();

        let after = CpuSnapshot::from(&cpu);
        assert_eq!(after.pc, before.pc.wrapping_add(1));
    }
}
