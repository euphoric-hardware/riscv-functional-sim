use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn mulhu_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = (cpu.load(rs1) as u128 * cpu.load(rs2) as u128) >> 64;
    cpu.store(rd, result as u64);
    Ok(cpu.pc + 4)
}