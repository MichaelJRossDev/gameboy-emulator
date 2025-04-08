use thiserror::Error;

use crate::{
    instruction::{
        Instruction, InstructionDecodeError, InstructionExecuteError, Opcode, OpcodeDecodeError,
    },
    memory::{FlatMemory, MemoryBus},
};

pub struct Cpu {
    pc: u16,
    sp: u16,
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    memory: Box<dyn MemoryBus>,
}

#[derive(Debug, Error)]
pub enum CpuStepError {
    #[error("Failed to decode opcode: {0}")]
    Opcode(#[from] OpcodeDecodeError),

    #[error("Failed to decode instruction: {0}")]
    Decode(#[from] InstructionDecodeError),

    #[error("Failed to execute instruction: {0}")]
    Execute(#[from] InstructionExecuteError),
}

impl Cpu {
    pub fn fetch_byte(&mut self) -> u8 {
        let value = self.memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    pub fn new(memory: Box<dyn MemoryBus>) -> Self {
        Self {
            memory,
            pc: 0,
            sp: 0,
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn step(&mut self) -> Result<(), CpuStepError> {
        let opcode = Opcode::try_from(self.fetch_byte())?;
        let instruction = Instruction::decode(opcode, self)?;
        instruction.execute(self)?;
        Ok(())
    }
}

#[cfg(test)]
pub struct CpuSnapshot {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

#[cfg(test)]
impl Cpu {
    pub fn cpu_with_program(program: &[u8]) -> Cpu {
        let mut memory = FlatMemory::new();

        for (i, byte) in program.iter().enumerate() {
            memory.write(0x0100 + i as u16, *byte);
        }

        let mut cpu = Cpu::new(Box::new(memory));

        cpu.pc = 0x0100;
        cpu
    }
}

#[cfg(test)]
impl From<&Cpu> for CpuSnapshot {
    fn from(cpu: &Cpu) -> Self {
        Self {
            pc: cpu.pc,
            sp: cpu.sp,
            a: cpu.a,
            f: cpu.f,
            b: cpu.b,
            c: cpu.c,
            d: cpu.d,
            e: cpu.e,
            h: cpu.h,
            l: cpu.l,
        }
    }
}
