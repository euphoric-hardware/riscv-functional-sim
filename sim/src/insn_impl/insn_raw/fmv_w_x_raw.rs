use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fmv_w_x_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let result64 = Insn::f32_to_f64_raw(f32::from_bits(cpu.load(rs1) as u32));
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}