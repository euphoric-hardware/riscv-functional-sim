use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

pub fn fcvt_lu_d_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::fcvt_lu_d_raw::fcvt_lu_d_raw(cpu, cache_entry.rd,cache_entry.rs1,cache_entry.rm)
}