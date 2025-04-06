use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn sra_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    cpu.store(cache_entry.rd, (cpu.load(cache_entry.rs1) as i64).wrapping_shr(((cpu.load(cache_entry.rs2)) & 0x3f) as u32) as u64);
    Ok(cpu.pc + 4)
}