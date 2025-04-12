use crate::{cpu::Cpu, reg};

#[test]
fn ld_b_imm8_sets_b_register() {
    let mut cpu = Cpu::cpu_with_program(&[0x06, 0x42]);
    cpu.step().unwrap();
    assert_eq!(cpu.get_register8(reg!(B)), 0x42);
}
