use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn sub_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    cpu.store(cache_entry.rd, cpu.load(cache_entry.rs1).wrapping_sub(cpu.load(cache_entry.rs2)));
    Ok(cpu.pc + 4)
}