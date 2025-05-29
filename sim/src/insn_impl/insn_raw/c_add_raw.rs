use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn  c_add_raw(cpu: &mut Cpu, rd_rs1_n0: u64, c_rs2_n0: u64) -> cpu::Result<u64> {
    let rs1 = cpu.load(rd_rs1_n0);
    let result = cpu.load(rd_rs1_n0).wrapping_add(cpu.load(c_rs2_n0));
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}