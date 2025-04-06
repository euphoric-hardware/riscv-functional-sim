use crate::{
    bus::{Bus, Device}, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::UopCacheEntry
};

pub fn lw_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::lw_raw::lw_raw(cpu, bus, cache_entry.rd, cache_entry.rs1, cache_entry.imm_i)
}
