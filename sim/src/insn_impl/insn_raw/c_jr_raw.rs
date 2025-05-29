use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_jr_raw(cpu: &mut Cpu, rs1_n0: u64) -> cpu::Result<u64> {
    let new_pc = cpu.load(rs1_n0);
    Ok(new_pc)
}