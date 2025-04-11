use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

pub fn c_xor_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_xor_raw::c_xor_raw(cpu, cache_entry.rd_rs1_p,cache_entry.rs2_p)
}