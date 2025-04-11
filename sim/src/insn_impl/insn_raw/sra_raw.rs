use crate::{
    cpu::{self, Cpu, Insn}
};

pub fn sra_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    cpu.store(
        rd,
        (cpu.load(rs1) as i64).wrapping_shr(((cpu.load(rs2)) & 0x3f) as u32) as u64,
    );
    Ok(cpu.pc + 4)
}
