use crate::{
    bus::{Bus, Device},
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn lw_raw(cpu: &mut Cpu, bus: &mut Bus, rd: u64, rs1: u64, imm_i: u64) -> cpu::Result<u64> {
    let offset = Insn::sign_extend(imm_i, 12);

    let address = cpu.load(rs1).wrapping_add(offset as u64);
    let mut raw = [0; size_of::<i32>()];
    bus.read(address, &mut raw)?;
    let h = i32::from_le_bytes(raw);
    cpu.store(rd, h as u64); // check sign extension
    Ok(cpu.pc + 4)
}
