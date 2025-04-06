use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn addi_cached(cpu: &mut Cpu, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let signed_imm = Insn::sign_extend(cache_entry.imm_i, 12) as u64;
    cpu.store(cache_entry.rd, cpu.load(cache_entry.rs1).wrapping_add(signed_imm));
    Ok(cpu.pc + 4)
}