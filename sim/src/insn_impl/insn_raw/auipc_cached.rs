use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn auipc_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let value = cpu.pc.wrapping_add(cache_entry.imm_u);
    cpu.store(cache_entry.rd, value);
    Ok(cpu.pc + 4)
}