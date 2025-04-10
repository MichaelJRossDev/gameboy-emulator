use super::Cpu;

#[derive(Debug, Clone, PartialEq, Eq)]
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
