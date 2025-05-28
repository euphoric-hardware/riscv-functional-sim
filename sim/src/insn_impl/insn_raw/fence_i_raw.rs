use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fence_i_raw(cpu: &mut Cpu, imm12: u64, rs1: u64, rd: u64) -> cpu::Result<u64> {
    Ok(cpu.pc + 4)
}