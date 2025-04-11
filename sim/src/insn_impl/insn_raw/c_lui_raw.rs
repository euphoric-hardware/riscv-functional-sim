use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_lui_raw(cpu: &mut Cpu, rd_n2: u64, imm_c_lui: u64) -> cpu::Result<u64> {
    cpu.store(rd_n2, imm_c_lui as u64);
    Ok(cpu.pc + 2)
}