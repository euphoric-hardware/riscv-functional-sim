use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn srai_raw(cpu: &mut Cpu, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let shamt = imm_i & 0b111111;
    cpu.store(rd, (cpu.load(rs1) as i64).wrapping_shl(shamt as u32) as u64);
    Ok(cpu.pc + 4)
}