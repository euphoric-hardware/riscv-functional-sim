use simple_soft_float::{F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_d_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let result = f64::from_bits((cpu.fload(rs1) as f32).to_bits() as u64);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}