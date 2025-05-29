use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_andi_raw(cpu: &mut Cpu, rd_rs1_p: u64, imm_c_andi: u64) -> cpu::Result<u64> {
    let result = cpu.load(rd_rs1_p + 8) & imm_c_andi;
    cpu.store(rd_rs1_p + 8, result);
    Ok(cpu.pc + 2)
}