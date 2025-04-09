use std::convert::TryFrom;
use thiserror::Error;

pub enum Opcode {
    Nop,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeDecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Opcode::Nop),
            _ => Err(OpcodeDecodeError::InvalidOpcode(byte)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpcodeDecodeError {
    #[error("Invalid opcode: 0x{0:02X}")]
    InvalidOpcode(u8),
}

