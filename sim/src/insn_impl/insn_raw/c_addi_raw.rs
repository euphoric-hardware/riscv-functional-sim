use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_addi_raw(cpu: &mut Cpu, rd_rs1_n0: u64, imm_c_addi: u64) -> cpu::Result<u64> {
    let result = cpu.load(rd_rs1_n0).wrapping_add(imm_c_addi as u64);
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}