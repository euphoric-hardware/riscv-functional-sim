use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

pub fn c_jalr_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_jalr_raw::c_jalr_raw(cpu, cache_entry.c_rs1_n0)
}