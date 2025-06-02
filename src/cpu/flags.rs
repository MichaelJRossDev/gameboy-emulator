use super::Cpu;

pub enum Flag {
    Zero,
    Subtract,
    HalfCarry,
    Carry
}

impl Flag {
    pub fn bit_mask(&self) -> u8 {
        match self {
            Flag::Zero      => 0b10000000,
            Flag::Subtract  => 0b01000000,
            Flag::HalfCarry => 0b00100000,
            Flag::Carry     => 0b00010000
        }
    }
}

impl Cpu {
    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get_register8(reg!(F)) & flag.bit_mask() != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        self.set_register8(reg!(F), self.get_register8(reg!(F)) | flag.bit_mask());
    }
}
