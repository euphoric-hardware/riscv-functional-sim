use crate::{
    bus::{Bus, Device}, cpu::{self, Cpu, Insn}, insn_impl::{add, insn_raw}, uop_cache::uop_cache::UopCacheEntry,
};

pub fn sd_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::sd_raw::sd_raw(cpu, bus, cache_entry.rs1, cache_entry.rs2, cache_entry.imm_s)
}