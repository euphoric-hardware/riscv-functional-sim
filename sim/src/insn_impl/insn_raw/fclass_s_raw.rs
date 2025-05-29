use simple_soft_float::F32;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fclass_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let value = f32::from_bits(cpu.fload(rs1).to_bits() as u32);
    let classification = Insn::classify_f32(value);

    cpu.store(rd, classification as u64);
    Ok(cpu.pc + 4)
}