use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn sb_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(cache_entry.imm_i as u64, 12);

    let address = cpu.load(cache_entry.rs1).wrapping_add(offset as u64);
    bus.write(address, &(cpu.load(cache_entry.rs2) as u8).to_le_bytes())?;
    Ok(cpu.pc + 4)
}