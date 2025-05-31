use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn c_lwsp_raw(cpu: &mut Cpu, bus: &mut Bus, rd_n0: u64, imm_c_lwsp: u64) -> cpu::Result<u64> {
    let address = (cpu.load(2)).wrapping_add(imm_c_lwsp);
    let mut raw = [0; size_of::<i32>()];
    bus.read(address, &mut raw)?;
    cpu.store(rd_n0, i32::from_le_bytes(raw) as u64);
    Ok(cpu.pc + 2)
}