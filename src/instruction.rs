pub mod opcode;
pub mod operand;
pub mod execute;
pub mod decode;

#[cfg(test)]
mod tests;

use crate::cpu::Cpu;

pub use opcode::{Opcode, OpcodeDecodeError};
pub use operand::Operand;

pub struct Instruction {
    opcode: Opcode,
    operands: (Operand, Operand),
}


