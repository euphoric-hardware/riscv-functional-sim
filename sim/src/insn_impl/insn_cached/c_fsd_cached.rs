use crate::{bus::Bus, cpu::{self, Cpu, Insn}, insn_impl::insn_raw, uop_cache::uop_cache::UopCacheEntry};

#[inline(always)]
pub fn c_fsd_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    insn_raw::c_fsd_raw::c_fsd_raw(cpu, bus, cache_entry.rs1_p,cache_entry.rs2_p,cache_entry.imm_c_sd)
}