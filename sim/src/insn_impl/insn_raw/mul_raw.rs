use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn mul_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = cpu.load(rs1).wrapping_mul(cpu.load(rs2));
    cpu.store(rd, result);

    Ok(cpu.pc + 4)
}