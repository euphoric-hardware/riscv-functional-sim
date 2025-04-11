use crate::{
    bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry
};

pub fn bge_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::bge_raw::bge_raw(cpu, cache_entry.rs1, cache_entry.rs2, cache_entry.imm_b)
}
