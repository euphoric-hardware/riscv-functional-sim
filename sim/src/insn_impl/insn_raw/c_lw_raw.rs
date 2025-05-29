use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn c_lw_raw(cpu: &mut Cpu, bus: &mut Bus, rd_p: u64, rs1_p: u64, imm_c_lw: u64) -> cpu::Result<u64> {
    let address = (cpu.load(rs1_p + 8) as u64).wrapping_add(imm_c_lw);
    let mut raw = [0; size_of::<i32>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd_p + 8, i32::from_le_bytes(raw) as u64);
    Ok(cpu.pc + 2)
}