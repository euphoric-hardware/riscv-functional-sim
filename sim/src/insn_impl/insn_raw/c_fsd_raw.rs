use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn c_fsd_raw(cpu: &mut Cpu, bus: &mut Bus, rs1_p: u64, rs2_p: u64, imm_c_sd: u64) -> cpu::Result<u64> {
    let address = (cpu.load(rs1_p + 8) as u64).wrapping_add(imm_c_sd as u64);
    let result = *cpu.fload(rs2_p + 8).bits();
    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 2)
}