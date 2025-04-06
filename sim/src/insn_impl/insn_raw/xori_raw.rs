use crate::{
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn xori_raw(cpu: &mut Cpu, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let signed_imm = Insn::sign_extend(imm_i, 12) as u64;
    cpu.store(rd, cpu.load(rs1) ^ signed_imm);
    Ok(cpu.pc + 4)
}
