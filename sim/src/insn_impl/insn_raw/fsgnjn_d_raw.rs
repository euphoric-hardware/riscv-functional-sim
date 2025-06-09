

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fsgnjn_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let sign = !(cpu.fload(rs2).to_bits() >> 63) << 63;
    let result = f64::from_bits((cpu.fload(rs1).to_bits() & 0x7fffffffffffffff) | sign);

    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}