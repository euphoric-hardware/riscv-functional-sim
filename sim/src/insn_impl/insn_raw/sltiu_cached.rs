use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn sltiu_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let result = if (cpu.load(cache_entry.rs1) < cache_entry.imm_i) {
        1
    } else {
        0
    };
    cpu.store(cache_entry.rd, result);
    Ok(cpu.pc + 4)
}
