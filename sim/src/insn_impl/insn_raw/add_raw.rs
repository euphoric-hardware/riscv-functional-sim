use crate::{
    cpu::{self, Cpu}
};

#[inline(always)]
pub fn add_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    cpu.store(rd, cpu.load(rs1).wrapping_add(cpu.load(rs2)));
    Ok(cpu.pc + 4)
}