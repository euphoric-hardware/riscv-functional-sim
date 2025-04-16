use std::arch::asm;

use simple_soft_float::F64;

use crate::{bus::Bus, cpu::{self, Cpu, Insn, RoundingMode}};

pub fn fcvt_d_wu_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let result: f64 = cpu.fload(rs1) as u32 as f64;
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}