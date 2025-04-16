use crate::{
    cpu::{Cpu, registers::Register8},
    reg,
};

fn test_inc_r8(register: Register8, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register8(register, 0x41);

    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(register), 0x42);
}

fn test_dec_r8(register: Register8, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register8(register, 0x43);

    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(register), 0x42);
}

// Increment
#[test]
fn test_inc_a() {
    test_inc_r8(reg!(A), 0x3C);
}

#[test]
fn test_inc_b() {
    test_inc_r8(reg!(B), 0x04);
}

#[test]
fn test_inc_c() {
    test_inc_r8(reg!(C), 0x0C);
}

#[test]
fn test_inc_d() {
    test_inc_r8(reg!(D), 0x14);
}

#[test]
fn test_inc_e() {
    test_inc_r8(reg!(E), 0x1C);
}

#[test]
fn test_inc_h() {
    test_inc_r8(reg!(H), 0x24);
}

#[test]
fn test_inc_l() {
    test_inc_r8(reg!(L), 0x2C);
}

// Decrement
#[test]
fn test_dec_a() {
    test_dec_r8(reg!(A), 0x3D);
}

#[test]
fn test_dec_b() {
    test_dec_r8(reg!(B), 0x05);
}

#[test]
fn test_dec_c() {
    test_dec_r8(reg!(C), 0x0D);
}

#[test]
fn test_dec_d() {
    test_dec_r8(reg!(D), 0x15);
}

#[test]
fn test_dec_e() {
    test_dec_r8(reg!(E), 0x1D);
}

#[test]
fn test_dec_h() {
    test_dec_r8(reg!(H), 0x25);
}

#[test]
fn test_dec_l() {
    test_dec_r8(reg!(L), 0x2D);
}
