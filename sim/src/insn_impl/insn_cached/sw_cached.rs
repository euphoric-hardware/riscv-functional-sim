use crate::{
    bus::{Bus, Device}, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry
};

#[inline(always)]
pub fn sw_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::sw_raw::sw_raw(cpu, bus, cache_entry.rs1, cache_entry.rs2, cache_entry.imm_s)
}