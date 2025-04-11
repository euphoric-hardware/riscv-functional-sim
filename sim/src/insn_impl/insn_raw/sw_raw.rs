use crate::{
    bus::{Bus, Device}, cpu::{self, Cpu, Insn}
};

pub fn sw_raw(cpu: &mut Cpu, bus: &mut Bus, rs1: u64, rs2: u64, imm_s: u64) -> cpu::Result<u64> {
    let offset = imm_s;

    let address = cpu.load(rs1).wrapping_add(offset as u64);
    bus.write(address, &(cpu.load(rs2) as u32).to_le_bytes())?;
    Ok(cpu.pc + 4)
}