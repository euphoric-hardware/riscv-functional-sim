use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn bgeu_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = cache_entry.imm_b;

    if cpu.load(cache_entry.rs1) >= cpu.load(cache_entry.rs2) {
        Ok((cpu.pc.wrapping_add(offset)))
    } else {
        Ok(cpu.pc + 4)
    }
}
