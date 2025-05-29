use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn lbu_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(imm_i as u64, 12);
    
    let address = (cpu.load(rs1)).wrapping_add(offset as u64);
    let mut raw = [0; size_of::<u8>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd, u8::from_le_bytes(raw) as u64);
    Ok(cpu.pc + 4)
}
