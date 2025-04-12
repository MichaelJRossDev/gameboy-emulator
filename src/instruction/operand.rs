use crate::cpu::registers::{Register8, Register16};

pub enum Operand {
    None,
    Imm8(u8),
    Imm16(u16),
    Register8(Register8),
    Register16(Register16),
}
