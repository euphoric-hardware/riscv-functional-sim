use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

#[inline(always)]
pub fn fmv_x_w_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::fmv_x_w_raw::fmv_x_w_raw(cpu, cache_entry.rd,cache_entry.rs1)
}