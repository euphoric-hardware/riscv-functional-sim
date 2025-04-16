use crate::{bus::{Bus, Device}, cpu::{self, Cpu, Insn}};

pub fn fsd_raw(cpu: &mut Cpu, bus: &mut Bus, rs1: u64, rs2: u64, imm_s: u64) -> cpu::Result<u64> {
    let address = (cpu.load(rs1) as u64).wrapping_add(imm_s);
    let result = cpu.fload(rs2).to_bits();

    bus.write(address, &result.to_le_bytes());
    Ok(cpu.pc + 4)
}
