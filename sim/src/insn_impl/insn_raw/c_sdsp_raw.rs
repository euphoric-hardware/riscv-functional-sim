use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_sdsp_raw(cpu: &mut Cpu, bus: &mut Bus, c_rs2: u64, imm_c_sdsp: u64) -> cpu::Result<u64> {
    let address = cpu.load(2).wrapping_add(imm_c_sdsp);
    bus.write(address, &cpu.load(c_rs2).to_le_bytes())?;
    Ok(cpu.pc + 2)
}