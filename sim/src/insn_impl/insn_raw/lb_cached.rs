use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}, uop_cache::UopCacheEntry
};

pub fn lb_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(cache_entry.imm_i as u64, 12);

    let address = (cpu.load(cache_entry.rs1) as u64).wrapping_add(offset as u64);
    let mut raw = [0];
    bus.read(address, &mut raw)?;
    cpu.store(cache_entry.rd, (raw[0] as i8) as u64); // check sign extension, does casting the byte work?
    Ok(cpu.pc + 4)
}