use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn slt_cached(cpu: &mut Cpu, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let result = if ((cpu.load(cache_entry.rs1) as i64) < (cpu.load(cache_entry.rs2) as i64)) {
        1
    } else {
        0
    };
    cpu.store(cache_entry.rd, result);
    Ok(cpu.pc + 4)
}
