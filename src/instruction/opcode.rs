use std::convert::TryFrom;
use thiserror::Error;

#[repr(u8)]
pub enum Opcode {
    Nop = 0x00,
}

#[derive(Debug, Error)]
pub enum OpcodeDecodeError {
    #[error("Invalid opcode: 0x{0:02X}")]
    InvalidOpcode(u8),
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
