use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_ldsp_raw(cpu: &mut Cpu, bus: &mut Bus, rd_n0: u64, imm_c_ldsp: u64) -> cpu::Result<u64> {
    let mut raw = [0; size_of::<u64>()];
    bus.read(cpu.load(2).wrapping_add(imm_c_ldsp), &mut raw)?;
    cpu.store(rd_n0, u64::from_le_bytes(raw));
    Ok(cpu.pc + 2)
}