use crate::cpu::{Cpu, registers::Register8};

fn test_ld_r8_r8(dst: Register8, src: Register8, opcode: u8) {
    let value = 0x42;

    let mut cpu = Cpu::new_with_program(&[opcode]);

    cpu.set_register8(src, value);
    cpu.step().unwrap();

    assert_eq!(
        cpu.get_register8(dst),
        value,
        "LD {:?}, {:?} failed",
        dst,
        src
    );
}

#[test]
fn test_ld_b_b() {
    test_ld_r8_r8(Register8::B, Register8::B, 0x40);
}
#[test]
fn test_ld_b_c() {
    test_ld_r8_r8(Register8::B, Register8::C, 0x41);
}
#[test]
fn test_ld_b_d() {
    test_ld_r8_r8(Register8::B, Register8::D, 0x42);
}
#[test]
fn test_ld_b_e() {
    test_ld_r8_r8(Register8::B, Register8::E, 0x43);
}
#[test]
fn test_ld_b_h() {
    test_ld_r8_r8(Register8::B, Register8::H, 0x44);
}
#[test]
fn test_ld_b_l() {
    test_ld_r8_r8(Register8::B, Register8::L, 0x45);
}
#[test]
fn test_ld_b_a() {
    test_ld_r8_r8(Register8::B, Register8::A, 0x47);
}

#[test]
fn test_ld_c_b() {
    test_ld_r8_r8(Register8::C, Register8::B, 0x48);
}
#[test]
fn test_ld_c_c() {
    test_ld_r8_r8(Register8::C, Register8::C, 0x49);
}
#[test]
fn test_ld_c_d() {
    test_ld_r8_r8(Register8::C, Register8::D, 0x4A);
}
#[test]
fn test_ld_c_e() {
    test_ld_r8_r8(Register8::C, Register8::E, 0x4B);
}
#[test]
fn test_ld_c_h() {
    test_ld_r8_r8(Register8::C, Register8::H, 0x4C);
}
#[test]
fn test_ld_c_l() {
    test_ld_r8_r8(Register8::C, Register8::L, 0x4D);
}
#[test]
fn test_ld_c_a() {
    test_ld_r8_r8(Register8::C, Register8::A, 0x4F);
}

#[test]
fn test_ld_d_b() {
    test_ld_r8_r8(Register8::D, Register8::B, 0x50);
}
#[test]
fn test_ld_d_c() {
    test_ld_r8_r8(Register8::D, Register8::C, 0x51);
}
#[test]
fn test_ld_d_d() {
    test_ld_r8_r8(Register8::D, Register8::D, 0x52);
}
#[test]
fn test_ld_d_e() {
    test_ld_r8_r8(Register8::D, Register8::E, 0x53);
}
#[test]
fn test_ld_d_h() {
    test_ld_r8_r8(Register8::D, Register8::H, 0x54);
}
#[test]
fn test_ld_d_l() {
    test_ld_r8_r8(Register8::D, Register8::L, 0x55);
}
#[test]
fn test_ld_d_a() {
    test_ld_r8_r8(Register8::D, Register8::A, 0x57);
}

#[test]
fn test_ld_e_b() {
    test_ld_r8_r8(Register8::E, Register8::B, 0x58);
}
#[test]
fn test_ld_e_c() {
    test_ld_r8_r8(Register8::E, Register8::C, 0x59);
}
#[test]
fn test_ld_e_d() {
    test_ld_r8_r8(Register8::E, Register8::D, 0x5A);
}
#[test]
fn test_ld_e_e() {
    test_ld_r8_r8(Register8::E, Register8::E, 0x5B);
}
#[test]
fn test_ld_e_h() {
    test_ld_r8_r8(Register8::E, Register8::H, 0x5C);
}
#[test]
fn test_ld_e_l() {
    test_ld_r8_r8(Register8::E, Register8::L, 0x5D);
}
#[test]
fn test_ld_e_a() {
    test_ld_r8_r8(Register8::E, Register8::A, 0x5F);
}

#[test]
fn test_ld_h_b() {
    test_ld_r8_r8(Register8::H, Register8::B, 0x60);
}
#[test]
fn test_ld_h_c() {
    test_ld_r8_r8(Register8::H, Register8::C, 0x61);
}
#[test]
fn test_ld_h_d() {
    test_ld_r8_r8(Register8::H, Register8::D, 0x62);
}
#[test]
fn test_ld_h_e() {
    test_ld_r8_r8(Register8::H, Register8::E, 0x63);
}
#[test]
fn test_ld_h_h() {
    test_ld_r8_r8(Register8::H, Register8::H, 0x64);
}
#[test]
fn test_ld_h_l() {
    test_ld_r8_r8(Register8::H, Register8::L, 0x65);
}
#[test]
fn test_ld_h_a() {
    test_ld_r8_r8(Register8::H, Register8::A, 0x67);
}

#[test]
fn test_ld_l_b() {
    test_ld_r8_r8(Register8::L, Register8::B, 0x68);
}
#[test]
fn test_ld_l_c() {
    test_ld_r8_r8(Register8::L, Register8::C, 0x69);
}
#[test]
fn test_ld_l_d() {
    test_ld_r8_r8(Register8::L, Register8::D, 0x6A);
}
#[test]
fn test_ld_l_e() {
    test_ld_r8_r8(Register8::L, Register8::E, 0x6B);
}
#[test]
fn test_ld_l_h() {
    test_ld_r8_r8(Register8::L, Register8::H, 0x6C);
}
#[test]
fn test_ld_l_l() {
    test_ld_r8_r8(Register8::L, Register8::L, 0x6D);
}
#[test]
fn test_ld_l_a() {
    test_ld_r8_r8(Register8::L, Register8::A, 0x6F);
}

#[test]
fn test_ld_a_b() {
    test_ld_r8_r8(Register8::A, Register8::B, 0x78);
}
#[test]
fn test_ld_a_c() {
    test_ld_r8_r8(Register8::A, Register8::C, 0x79);
}
#[test]
fn test_ld_a_d() {
    test_ld_r8_r8(Register8::A, Register8::D, 0x7A);
}
#[test]
fn test_ld_a_e() {
    test_ld_r8_r8(Register8::A, Register8::E, 0x7B);
}
#[test]
fn test_ld_a_h() {
    test_ld_r8_r8(Register8::A, Register8::H, 0x7C);
}
#[test]
fn test_ld_a_l() {
    test_ld_r8_r8(Register8::A, Register8::L, 0x7D);
}
#[test]
fn test_ld_a_a() {
    test_ld_r8_r8(Register8::A, Register8::A, 0x7F);
}
