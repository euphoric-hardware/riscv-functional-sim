use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn slti_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let signed_imm = Insn::sign_extend(cache_entry.imm_i, 12) as u64;
    let result = if (cpu.load(cache_entry.rs1) < signed_imm) {
        1
    } else {
        0
    };
    cpu.store(cache_entry.rd, result);
    Ok(cpu.pc + 4)
}
