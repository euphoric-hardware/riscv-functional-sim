use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn jalr_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(cache_entry.imm_i, 12) as u64;

    cpu.store(cache_entry.rd, cpu.pc + 4);
    Ok(cpu.pc.wrapping_add(offset))
}