use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn},
    uop_cache::UopCacheEntry,
};

pub fn ld_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(imm_i as u64, 12);

    let address = (cpu.load(rs1) as u64).wrapping_add(offset as u64);
    let mut raw = [0; size_of::<u64>()];
    bus.read(address, &mut raw)?;
    let h = u64::from_le_bytes(raw);
    cpu.store(rd, h);
    Ok(cpu.pc + 4)
}
