use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_sw_raw(cpu: &mut Cpu, bus: &mut Bus, rs1_p: u64, rs2_p: u64, imm_c_sw: u64) -> cpu::Result<u64> {
    let address = cpu.load(rs1_p + 8).wrapping_add(imm_c_sw);
    bus.write(address, &(cpu.load(rs2_p + 8) as u32).to_le_bytes())?;
    Ok(cpu.pc + 2)
}