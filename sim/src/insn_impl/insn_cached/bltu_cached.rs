use crate::{
    bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw::{self, bltu_raw}, uop_cache::UopCacheEntry
};

pub fn bltu_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::bltu_raw::bltu_raw(cpu, cache_entry.rs1, cache_entry.rs2, cache_entry.imm_b)
}
