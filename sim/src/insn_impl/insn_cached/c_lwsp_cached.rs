use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

#[inline(always)]
pub fn c_lwsp_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_lwsp_raw::c_lwsp_raw(cpu, bus, cache_entry.rd_n0,cache_entry.imm_c_lwsp)
}