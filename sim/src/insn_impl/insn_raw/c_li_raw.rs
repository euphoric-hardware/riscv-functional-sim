use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_li_raw(cpu: &mut Cpu, rd_n0: u64, imm_c_li: u64) -> cpu::Result<u64> {
    cpu.store(rd_n0, imm_c_li as u64);
    Ok(cpu.pc + 2)
}