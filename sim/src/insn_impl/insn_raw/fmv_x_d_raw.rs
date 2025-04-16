use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_x_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let result = cpu.fload(rs1).to_bits();
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}