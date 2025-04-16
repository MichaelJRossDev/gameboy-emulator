use crate::util;

use super::Cpu;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[macro_export]
macro_rules! reg {
    (A) => {
        $crate::cpu::registers::Register8::A
    };
    (F) => {
        $crate::cpu::registers::Register8::F
    };
    (B) => {
        $crate::cpu::registers::Register8::B
    };
    (C) => {
        $crate::cpu::registers::Register8::C
    };
    (D) => {
        $crate::cpu::registers::Register8::D
    };
    (E) => {
        $crate::cpu::registers::Register8::E
    };
    (H) => {
        $crate::cpu::registers::Register8::H
    };
    (L) => {
        $crate::cpu::registers::Register8::L
    };

    (AF) => {
        $crate::cpu::registers::Register16::AF
    };
    (BC) => {
        $crate::cpu::registers::Register16::BC
    };
    (DE) => {
        $crate::cpu::registers::Register16::DE
    };
    (HL) => {
        $crate::cpu::registers::Register16::HL
    };
    (SP) => {
        $crate::cpu::registers::Register16::SP
    };
    (PC) => {
        $crate::cpu::registers::Register16::PC
    };
}

impl Cpu {
    pub fn get_register8(&self, reg: Register8) -> u8 {
        match reg {
            Register8::A => self.a,
            Register8::F => self.f,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn set_register8(&mut self, reg: Register8, value: u8) {
        match reg {
            Register8::A => self.a = value,
            Register8::F => self.f = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    pub fn get_register16(&self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => util::u16_from_little_endian(
                self.get_register8(reg!(A)),
                self.get_register8(reg!(F)),
            ),
            Register16::BC => util::u16_from_little_endian(
                self.get_register8(reg!(B)),
                self.get_register8(reg!(C)),
            ),
            Register16::DE => util::u16_from_little_endian(
                self.get_register8(reg!(D)),
                self.get_register8(reg!(E)),
            ),
            Register16::HL => util::u16_from_little_endian(
                self.get_register8(reg!(H)),
                self.get_register8(reg!(L)),
            ),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    pub fn set_register16(&mut self, reg: Register16, value: u16) {
        let little_endian = util::little_endian_from_u16(value);

        match reg {
            Register16::AF => {
                self.set_register8(reg!(A), little_endian.0);
                self.set_register8(reg!(F), little_endian.1);
            }
            Register16::BC => {
                self.set_register8(reg!(B), little_endian.0);
                self.set_register8(reg!(C), little_endian.1);
            }
            Register16::DE => {
                self.set_register8(reg!(D), little_endian.0);
                self.set_register8(reg!(E), little_endian.1);
            }
            Register16::HL => {
                self.set_register8(reg!(H), little_endian.0);
                self.set_register8(reg!(L), little_endian.1);
            }
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        }
    }

    pub fn borrow_mutable_r8(&mut self, reg: Register8) -> &mut u8 {
        match reg {
            Register8::A => &mut self.a,
            Register8::F => &mut self.f,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
        }
    }

    pub fn add_r8(&mut self, reg: Register8, value: u8) {
        self.set_register8(reg, self.get_register8(reg).wrapping_add(value));
    }

    pub fn sub_r8(&mut self, reg: Register8, value: u8) {
        self.set_register8(reg, self.get_register8(reg).wrapping_sub(value));
    }

    pub fn add_r16(&mut self, reg: Register16, value: u16) {
        self.set_register16(reg, self.get_register16(reg).wrapping_add(value));
    }

    pub fn sub_r16(&mut self, reg: Register16, value: u16) {
        self.set_register16(reg, self.get_register16(reg).wrapping_sub(value));
    }
}
