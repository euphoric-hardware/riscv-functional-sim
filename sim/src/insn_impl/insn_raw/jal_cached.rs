use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn jal_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = cache_entry.imm_j;

    cpu.store(cache_entry.rd, cpu.pc + 4);
    Ok(cpu.pc.wrapping_add(offset))
}