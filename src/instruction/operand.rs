use crate::cpu::registers::{Register8, Register16};

pub enum Operand {
    None,
    Imm8(u8),
    Imm16(u16),
    Register8(Register8),
    Register16(Register16),
    Address(u16)
}

#[macro_export]
macro_rules! unpack_operand {
    ($op:expr, Imm8) => {
        if let Operand::Imm8(val) = $op {
            *val
        } else {
            panic!("Expected Operand::Imm8");
        }
    };
    ($op:expr, Imm16) => {
        if let Operand::Imm16(val) = $op {
            *val
        } else {
            panic!("Expected Operand::Imm16");
        }
    };
    ($op:expr, Register8) => {
        if let Operand::Register8(val) = $op {
            *val
        } else {
            panic!("Expected Operand::Register8");
        }
    };
    ($op:expr, Register16) => {
        if let Operand::Register16(val) = $op {
            *val
        } else {
            panic!("Expected Operand::Register16");
        }
    };
    ($op:expr, Address) => {
        if let Operand::Address(val) = $op {
            *val
        } else {
            panic!("Expected Operand::Address");
        }
    };
    ($op:expr, None) => {
        if let Operand::None = $op {
            None
        } else {
            panic!("Expected Operand::None");
        }
    };
}

