use crate::{
    bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::UopCacheEntry
};

pub fn addiw_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::addiw_raw::addiw_raw(cpu, cache_entry.rd, cache_entry.rs1, cache_entry.imm_i)
}