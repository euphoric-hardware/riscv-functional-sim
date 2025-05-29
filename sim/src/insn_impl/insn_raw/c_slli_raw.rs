use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_slli_raw(cpu: &mut Cpu, rd_rs1_n0: u64, imm_c_shamt: u64) -> cpu::Result<u64> {
    let result = cpu.load(rd_rs1_n0).wrapping_shl(imm_c_shamt as u32);
    cpu.store(rd_rs1_n0, result);
    Ok(cpu.pc + 2)
}