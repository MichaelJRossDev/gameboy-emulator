use super::Cpu;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register8 {
    A,
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
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    pub fn borrow_mutable_r8(&mut self, reg: Register8) -> &mut u8 {
        match reg {
            Register8::A => &mut self.a,
            Register8::B => &mut self.b,
            Register8::C => &mut self.c,
            Register8::D => &mut self.d,
            Register8::E => &mut self.e,
            Register8::H => &mut self.h,
            Register8::L => &mut self.l,
        }
    }

    pub fn add_r8(&mut self, reg: Register8, value: u8) {
        self.set_register8(reg, self.get_register8(reg) + value);
    }

    pub fn sub_r8(&mut self, reg: Register8, value: u8) {
        self.set_register8(reg, self.get_register8(reg) - value);
    }
}
