use crate::{
    bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw::{self, and_raw}, uop_cache::UopCacheEntry
};

pub fn and_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::and_raw::and_raw(cpu, cache_entry.rd, cache_entry.rs1, cache_entry.rs2)
}