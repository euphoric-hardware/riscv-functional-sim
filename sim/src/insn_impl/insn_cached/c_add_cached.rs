use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

#[inline(always)]
pub fn c_add_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_add_raw::c_add_raw(cpu, cache_entry.rd_rs1_n0,cache_entry.c_rs2_n0)
}