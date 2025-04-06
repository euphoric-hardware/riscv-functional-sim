use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn nop_cached(cpu: &mut Cpu, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    Ok(cpu.pc + 4)
}