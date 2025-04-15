use crate::cpu::{registers::Register8, Cpu};

#[test]
fn test_ld_a_from_a16() {
    let target_addr = 0x1234;
    let value = 0xAB;
    let program = [0xFA, 0x34, 0x12];

    let mut cpu = Cpu::new_with_program(&program);

    cpu.write(target_addr, value);

    assert_ne!(cpu.get_register8(Register8::A), value);

    cpu.step().unwrap();

    assert_eq!(cpu.get_register8(Register8::A), value);
}

