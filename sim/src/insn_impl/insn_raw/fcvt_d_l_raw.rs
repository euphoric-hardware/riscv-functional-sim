use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fcvt_d_l_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rm: u64) -> cpu::Result<u64> {
    let result = F64::from_i64(cpu.load(rs1) as i64, None, None);
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}