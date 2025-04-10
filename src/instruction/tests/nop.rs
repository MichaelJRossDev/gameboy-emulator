use crate::cpu::{Cpu, snapshot::CpuSnapshot};

#[test]
fn nop_only_advances_pc() {
    let mut cpu = Cpu::cpu_with_program(&[0x00]);
    let before = CpuSnapshot::from(&cpu);

    cpu.step().unwrap();

    let expected = CpuSnapshot {
        pc: before.pc.wrapping_add(1),
        ..before
    };

    let after = CpuSnapshot::from(&cpu);

    assert_eq!(after, expected);
}
