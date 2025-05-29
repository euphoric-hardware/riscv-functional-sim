use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

#[inline(always)]
pub fn fadd_d_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::fadd_d_raw::fadd_d_raw(cpu, cache_entry.rd,cache_entry.rs1,cache_entry.rs2,cache_entry.rm)
}