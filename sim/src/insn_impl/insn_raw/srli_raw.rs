use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn srli_raw(cpu: &mut Cpu, rd: u64, rs1: u64, shamtd: u64) -> cpu::Result<u64> {
    cpu.store(rd, cpu.load(rs1).wrapping_shr(shamtd as u32));
    Ok(cpu.pc + 4)
}