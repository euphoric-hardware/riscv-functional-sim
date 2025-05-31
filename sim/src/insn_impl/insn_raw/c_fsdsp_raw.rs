use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

#[inline(always)]
pub fn c_fsdsp_raw(cpu: &mut Cpu, bus: &mut Bus, c_rs2: u64, imm_c_sdsp: u64) -> cpu::Result<u64> {
    let address = cpu.load(2).wrapping_add(imm_c_sdsp);
    let result = cpu.fload(c_rs2);
    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 2)
}