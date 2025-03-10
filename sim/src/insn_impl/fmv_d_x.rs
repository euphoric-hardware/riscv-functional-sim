use simple_soft_float::F64;

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_d_x(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();

    let result = F64::from_bits(cpu.load(rs1));
    cpu.fstore(rd, result);
    Ok(cpu.pc + 4)
}