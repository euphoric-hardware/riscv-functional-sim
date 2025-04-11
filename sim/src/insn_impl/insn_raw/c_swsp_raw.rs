use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_swsp_raw(cpu: &mut Cpu, bus: &mut Bus, c_rs2: u64, imm_c_swsp: u64) -> cpu::Result<u64> {
    let address = cpu.load(2).wrapping_add(imm_c_swsp as u64);
    bus.write(address, &(cpu.load(c_rs2) as u32).to_le_bytes())?;
    Ok(cpu.pc + 2)
}