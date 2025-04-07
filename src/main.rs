// Temporary for early development
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cpu::Cpu;
use instruction::{Instruction, Opcode};
use memory::FlatMemory;

mod instruction;
mod memory;
mod cpu;

fn main() {
    println!("Hello, world!");
    let mut new_cpu = Cpu::new(Box::new(FlatMemory::new()));
    let new_instruction = Instruction::decode(Opcode::Nop, &mut new_cpu).unwrap();
    new_instruction.execute(&mut new_cpu);
}
