use std::convert::TryFrom;
use thiserror::Error;

pub enum Opcode {
    //Nop
    Nop,

    //Jump
    JpImm16,

    //Ld r8 imm8
    LdBImm8,
    LdCImm8,
    LdDImm8,
    LdEImm8,
    LdHImm8,
    LdLImm8,
    LdAImm8,

    //Ld r8 r8
    LdBB,
    LdBC,
    LdBD,
    LdBE,
    LdBH,
    LdBL,
    LdBA,
    LdCB,
    LdCC,
    LdCD,
    LdCE,
    LdCH,
    LdCL,
    LdCA,
    LdDB,
    LdDC,
    LdDD,
    LdDE,
    LdDH,
    LdDL,
    LdDA,
    LdEB,
    LdEC,
    LdED,
    LdEE,
    LdEH,
    LdEL,
    LdEA,
    LdHB,
    LdHC,
    LdHD,
    LdHE,
    LdHH,
    LdHL,
    LdHA,
    LdLB,
    LdLC,
    LdLD,
    LdLE,
    LdLH,
    LdLL,
    LdLA,
    LdAB,
    LdAC,
    LdAD,
    LdAE,
    LdAH,
    LdAL,
    LdAA,

    // INC r8
    IncA,
    IncB,
    IncC,
    IncD,
    IncE,
    IncH,
    IncL,

    // DEC r8
    DecA,
    DecB,
    DecC,
    DecD,
    DecE,
    DecH,
    DecL,

    // INC r16
    IncBC,
    IncDE,
    IncHL,
    IncSP,

    // DEC r16
    DecBC,
    DecDE,
    DecHL,
    DecSP,

    LdMemHLB,
    LdMemHLC,
    LdMemHLD,
    LdMemHLE,
    LdMemHLH,
    LdMemHLL,
    LdMemHLA,
    LdBFromMemHL,
    LdCFromMemHL,
    LdDFromMemHL,
    LdEFromMemHL,
    LdHFromMemHL,
    LdLFromMemHL,
    LdAFromMemHL,

    LdAFromAddr,
    LdAddrA,
}

impl TryFrom<u8> for Opcode {
    type Error = OpcodeDecodeError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            // Nop
            0x00 => Ok(Opcode::Nop),

            // Jump
            0xC3 => Ok(Opcode::JpImm16),

            // Ld r8 imm8
            0x06 => Ok(Opcode::LdBImm8),
            0x0E => Ok(Opcode::LdCImm8),
            0x16 => Ok(Opcode::LdDImm8),
            0x1E => Ok(Opcode::LdEImm8),
            0x26 => Ok(Opcode::LdHImm8),
            0x2E => Ok(Opcode::LdLImm8),
            0x3E => Ok(Opcode::LdAImm8),

            // Ld r8 r8
            0x40 => Ok(Opcode::LdBB),
            0x41 => Ok(Opcode::LdBC),
            0x42 => Ok(Opcode::LdBD),
            0x43 => Ok(Opcode::LdBE),
            0x44 => Ok(Opcode::LdBH),
            0x45 => Ok(Opcode::LdBL),
            0x47 => Ok(Opcode::LdBA),
            0x48 => Ok(Opcode::LdCB),
            0x49 => Ok(Opcode::LdCC),
            0x4A => Ok(Opcode::LdCD),
            0x4B => Ok(Opcode::LdCE),
            0x4C => Ok(Opcode::LdCH),
            0x4D => Ok(Opcode::LdCL),
            0x4F => Ok(Opcode::LdCA),
            0x50 => Ok(Opcode::LdDB),
            0x51 => Ok(Opcode::LdDC),
            0x52 => Ok(Opcode::LdDD),
            0x53 => Ok(Opcode::LdDE),
            0x54 => Ok(Opcode::LdDH),
            0x55 => Ok(Opcode::LdDL),
            0x57 => Ok(Opcode::LdDA),
            0x58 => Ok(Opcode::LdEB),
            0x59 => Ok(Opcode::LdEC),
            0x5A => Ok(Opcode::LdED),
            0x5B => Ok(Opcode::LdEE),
            0x5C => Ok(Opcode::LdEH),
            0x5D => Ok(Opcode::LdEL),
            0x5F => Ok(Opcode::LdEA),
            0x60 => Ok(Opcode::LdHB),
            0x61 => Ok(Opcode::LdHC),
            0x62 => Ok(Opcode::LdHD),
            0x63 => Ok(Opcode::LdHE),
            0x64 => Ok(Opcode::LdHH),
            0x65 => Ok(Opcode::LdHL),
            0x67 => Ok(Opcode::LdHA),
            0x68 => Ok(Opcode::LdLB),
            0x69 => Ok(Opcode::LdLC),
            0x6A => Ok(Opcode::LdLD),
            0x6B => Ok(Opcode::LdLE),
            0x6C => Ok(Opcode::LdLH),
            0x6D => Ok(Opcode::LdLL),
            0x6F => Ok(Opcode::LdLA),
            0x78 => Ok(Opcode::LdAB),
            0x79 => Ok(Opcode::LdAC),
            0x7A => Ok(Opcode::LdAD),
            0x7B => Ok(Opcode::LdAE),
            0x7C => Ok(Opcode::LdAH),
            0x7D => Ok(Opcode::LdAL),
            0x7F => Ok(Opcode::LdAA),

            // Accumulator <=> Address
            0xFA => Ok(Opcode::LdAFromAddr),
            0xEA => Ok(Opcode::LdAddrA),

            // Inc r8
            0x3C => Ok(Opcode::IncA),
            0x04 => Ok(Opcode::IncB),
            0x0C => Ok(Opcode::IncC),
            0x14 => Ok(Opcode::IncD),
            0x1C => Ok(Opcode::IncE),
            0x24 => Ok(Opcode::IncH),
            0x2C => Ok(Opcode::IncL),

            //Dec r8
            0x3D => Ok(Opcode::DecA),
            0x05 => Ok(Opcode::DecB),
            0x0D => Ok(Opcode::DecC),
            0x15 => Ok(Opcode::DecD),
            0x1D => Ok(Opcode::DecE),
            0x25 => Ok(Opcode::DecH),
            0x2D => Ok(Opcode::DecL),

            // Inc r16
            0x03 => Ok(Opcode::IncBC),
            0x13 => Ok(Opcode::IncDE),
            0x23 => Ok(Opcode::IncHL),
            0x33 => Ok(Opcode::IncSP),

            // Dec r16
            0x0B => Ok(Opcode::DecBC),
            0x1B => Ok(Opcode::DecDE),
            0x2B => Ok(Opcode::DecHL),
            0x3B => Ok(Opcode::DecSP),

            // Ld r8 MemHL
            0x46 => Ok(Opcode::LdBFromMemHL),
            0x4E => Ok(Opcode::LdCFromMemHL),
            0x56 => Ok(Opcode::LdDFromMemHL),
            0x5E => Ok(Opcode::LdEFromMemHL),
            0x66 => Ok(Opcode::LdHFromMemHL),
            0x6E => Ok(Opcode::LdLFromMemHL),
            0x7E => Ok(Opcode::LdAFromMemHL),

            // Write r8 to MemHL
            // 0x70 => Ok(Opcode::LdMemHLB),
            // 0x71 => Ok(Opcode::LdMemHLC),
            // 0x72 => Ok(Opcode::LdMemHLD),
            // 0x73 => Ok(Opcode::LdMemHLE),
            // 0x74 => Ok(Opcode::LdMemHLH),
            // 0x75 => Ok(Opcode::LdMemHLL),
            // 0x77 => Ok(Opcode::LdMemHLA),

            _ => Err(OpcodeDecodeError::InvalidOpcode(byte)),
        }
    }
}

#[derive(Debug, Error)]
pub enum OpcodeDecodeError {
    #[error("Invalid opcode: 0x{0:02X}")]
    InvalidOpcode(u8),
}
