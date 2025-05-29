use crate::{
    cpu::{self, Cpu, Insn}
};

#[inline(always)]
pub fn slt_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = if (cpu.load(rs1) as i64) < (cpu.load(rs2) as i64) {
        1
    } else {
        0
    };
    cpu.store(rd, result);
    Ok(cpu.pc + 4)
}
