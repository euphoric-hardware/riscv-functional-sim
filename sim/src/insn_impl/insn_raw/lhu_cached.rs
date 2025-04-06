use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn lhu_cached(cpu: &mut Cpu, bus: &mut Bus, cache_entry: &UopCacheEntry) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(cache_entry.imm_i as u64, 12);

    let address = (cpu.load(cache_entry.rs1) as u64).wrapping_add(offset as u64);
    let mut raw = [0; size_of::<u16>()];
    bus.read(address, &mut raw)?;
    let h = u16::from_le_bytes(raw);
    cpu.store(cache_entry.rd, h as u64);
    Ok(cpu.pc + 4)
}
