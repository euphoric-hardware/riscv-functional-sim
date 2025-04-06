use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn slli_cached(cpu: &mut Cpu, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let shamt = cache_entry.imm_i & 0b111111;
    cpu.store(cache_entry.rd, cpu.load(cache_entry.rs1).wrapping_shl(shamt as u32));
    Ok(cpu.pc + 4)
}