use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_w_x_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let result64 = f64::from_bits(0xffffffff00000000 | cpu.load(rs1));
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}