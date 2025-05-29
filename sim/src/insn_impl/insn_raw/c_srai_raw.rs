use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn c_srai_raw(cpu: &mut Cpu, rd_rs1_p: u64, imm_c_shamt: u64) -> cpu::Result<u64> {
    let result = (cpu.load(rd_rs1_p + 8) as i64).wrapping_shr(imm_c_shamt as u32) as u64;
    cpu.store(rd_rs1_p + 8, result);
    Ok(cpu.pc + 2)
}