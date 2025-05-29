use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn fclass_d_raw(cpu: &mut Cpu, rd: u64, rs1: u64) -> cpu::Result<u64> {
    let value = cpu.fload(rs1);
    let classification = Insn::classify_f64(value);
    cpu.store(rd, classification as u64);
    Ok(cpu.pc + 4)
}