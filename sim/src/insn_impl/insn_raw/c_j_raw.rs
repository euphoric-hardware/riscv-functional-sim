use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_j_raw(cpu: &mut Cpu, imm_c_j: u64) -> cpu::Result<u64> {
    let new_pc = cpu.pc.wrapping_add(imm_c_j);
    Ok(new_pc as u64)
}