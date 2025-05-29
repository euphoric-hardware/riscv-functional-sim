use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn mulh_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result: i128 = (cpu.load(rs1) as i64 as i128 * cpu.load(rs2) as i64 as i128) >> 64;
    cpu.store(rd, result as u64);
    Ok(cpu.pc + 4)
}