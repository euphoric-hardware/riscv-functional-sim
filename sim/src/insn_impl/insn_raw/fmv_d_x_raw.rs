use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_d_x_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let result = f64::from_bits(cpu.load(rs1));
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}