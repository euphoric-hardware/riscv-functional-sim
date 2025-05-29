use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn bgeu_raw(cpu: &mut Cpu, rs1: u64, rs2: u64, imm_b: u64) -> cpu::Result<u64> {
    let offset = imm_b;

    if cpu.load(rs1) >= cpu.load(rs2) {
        Ok(cpu.pc.wrapping_add(offset))
    } else {
        Ok(cpu.pc + 4)
    }
}
