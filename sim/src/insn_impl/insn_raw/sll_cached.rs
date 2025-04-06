use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn sll_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    cpu.store(cache_entry.rd, cpu.load(cache_entry.rs1).wrapping_shl(((cpu.load(cache_entry.rs2)) & 0x3f) as u32));
    Ok(cpu.pc + 4)
}