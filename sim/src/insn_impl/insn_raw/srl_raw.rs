use crate::{
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn srl_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    cpu.store(
        rd,
        cpu.load(rs1).wrapping_shr(((cpu.load(rs2)) & 0x3f) as u32),
    );
    Ok(cpu.pc + 4)
}
