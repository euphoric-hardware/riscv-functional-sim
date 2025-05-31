use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn c_ld_raw(cpu: &mut Cpu, bus: &mut Bus, rd_p: u64, rs1_p: u64, imm_c_ld: u64) -> cpu::Result<u64> {
    let address = cpu.load(rs1_p + 8).wrapping_add(imm_c_ld);
    let mut raw = [0; size_of::<u64>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd_p + 8, u64::from_le_bytes(raw));
    Ok(cpu.pc + 2)
}