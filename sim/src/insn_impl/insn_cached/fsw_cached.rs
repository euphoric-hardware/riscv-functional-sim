use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

pub fn fsw_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::fsw_raw::fsw_raw(cpu, bus, cache_entry.rd, cache_entry.rs1, cache_entry.rs2, cache_entry.imm_s)
}