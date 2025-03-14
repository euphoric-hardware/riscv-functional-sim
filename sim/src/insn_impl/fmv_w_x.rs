use simple_soft_float::{F32, F64};

use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmv_w_x(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    
    let result64 = F64::from_bits(0xffffffff00000000 | cpu.load(rs1));
    cpu.fstore(rd, result64);
    Ok(cpu.pc + 4)
}