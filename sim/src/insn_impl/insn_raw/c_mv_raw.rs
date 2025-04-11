use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_mv_raw(cpu: &mut Cpu, rd_n0: u64, c_rs2_n0: u64) -> cpu::Result<u64> {
    cpu.store(rd_n0, cpu.load(c_rs2_n0));
    Ok(cpu.pc + 2)
}