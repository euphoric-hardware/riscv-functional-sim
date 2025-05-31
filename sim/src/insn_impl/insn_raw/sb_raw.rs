use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn sb_raw(cpu: &mut Cpu, bus: &mut Bus, rs1: u64, rs2: u64, imm_s: u64) -> cpu::Result<u64> {
    let offset: u64 = imm_s;

    let address = cpu.load(rs1).wrapping_add(offset);
    bus.write(address, &(cpu.load(rs2) as u8).to_le_bytes())?;
    Ok(cpu.pc + 4)
}