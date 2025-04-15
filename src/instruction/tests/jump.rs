use crate::cpu::Cpu;

#[test]
fn test_jp_imm16() {
    let program = [0xC3, 0x34, 0x12];
    let mut cpu = Cpu::new_with_program(&program);
    assert!(
        cpu.pc() == 0x0100,
        "Expected PC to be 0x0100, but got 0x{:04X}",
        cpu.pc()
    );

    cpu.step().unwrap();

    assert!(
        cpu.pc() == 0x1234,
        "Expected PC to be 0x1234, but got 0x{:04X}",
        cpu.pc(),
    );
}
