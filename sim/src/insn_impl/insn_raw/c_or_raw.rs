use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_or_raw(cpu: &mut Cpu, rd_rs1_p: u64, rs2_p: u64) -> cpu::Result<u64> {
    let result = cpu.load(rd_rs1_p + 8) | cpu.load(rs2_p + 8);
    cpu.store(rd_rs1_p + 8, result);
    Ok(cpu.pc + 2)
}