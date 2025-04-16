use crate::{
    cpu::{Cpu, registers::Register16},
    reg,
};

fn test_inc_r16(register: Register16, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register16(register, 0x1233);

    cpu.step().unwrap();
    assert!(
        cpu.get_register16(register) == 0x1234,
        "Expected register to be 0x1234, but got 0x{:04X}",
        cpu.get_register16(register)
    );
}

fn test_dec_r16(register: Register16, opcode: u8) {
    let mut cpu = Cpu::new_with_program(&[opcode]);
    cpu.set_register16(register, 0x1235);

    cpu.step().unwrap();
    assert!(
        cpu.get_register16(register) == 0x1234,
        "Expected register to be 0x1234, but got 0x{:04X}",
        cpu.get_register16(register)
    );
}

// Increment
#[test]
fn test_inc_bc() {
    test_inc_r16(reg!(BC), 0x03);
}

#[test]
fn test_inc_de() {
    test_inc_r16(reg!(DE), 0x13);
}

#[test]
fn test_inc_hl() {
    test_inc_r16(reg!(HL), 0x23);
}

#[test]
fn test_inc_sp() {
    test_inc_r16(reg!(SP), 0x33);
}

// Decrement
#[test]
fn test_dec_bc() {
    test_dec_r16(reg!(BC), 0x0B);
}

#[test]
fn test_dec_de() {
    test_dec_r16(reg!(DE), 0x1B);
}

#[test]
fn test_dec_hl() {
    test_dec_r16(reg!(HL), 0x2B);
}

#[test]
fn test_dec_sp() {
    test_dec_r16(reg!(SP), 0x3B);
}
