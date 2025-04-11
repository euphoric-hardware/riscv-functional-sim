use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

pub fn c_mv_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_mv_raw::c_mv_raw(cpu, cache_entry.rd_n0,cache_entry.c_rs2_n0)
}