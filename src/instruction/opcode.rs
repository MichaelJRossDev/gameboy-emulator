use std::convert::TryFrom;
use thiserror::Error;

pub enum Opcode {
    Nop,
    LdBImm8,
    LdCImm8,
    LdDImm8,
    LdEImm8,
    LdHImm8,
    LdLImm8,
    LdAImm8,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeDecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Opcode::Nop),
            0x06 => Ok(Opcode::LdBImm8),
            0x0E => Ok(Opcode::LdCImm8),
            0x16 => Ok(Opcode::LdDImm8),
            0x1E => Ok(Opcode::LdEImm8),
            0x26 => Ok(Opcode::LdHImm8),
            0x2E => Ok(Opcode::LdLImm8),
            0x3E => Ok(Opcode::LdAImm8),
            _ => Err(OpcodeDecodeError::InvalidOpcode(byte)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpcodeDecodeError {
    #[error("Invalid opcode: 0x{0:02X}")]
    InvalidOpcode(u8),
}

