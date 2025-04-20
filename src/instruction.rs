pub mod opcode;
pub mod operand;
pub mod execute;
pub mod decode;

pub use opcode::{Opcode, OpcodeDecodeError};
pub use decode::InstructionDecodeError;
pub use execute::InstructionExecuteError;
pub use operand::Operand;

#[cfg(test)]
mod tests;

pub struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

