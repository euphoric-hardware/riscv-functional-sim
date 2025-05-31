use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fsgnj_s_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let sign = (cpu.fload(rs2).to_bits() as u32) & 0x80000000;
    let result = (cpu.fload(rs1).to_bits() as u32) | sign;
    let result64 = f64::from_bits(0xffffffff00000000 | (result as u64));
    
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}