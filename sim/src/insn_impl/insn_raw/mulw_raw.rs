use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn mulw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let result = Insn::sign_extend(
        (cpu.load(rs1) as u32).wrapping_mul(cpu.load(rs2) as u32) as u32 as u64,
        32,
    );
    cpu.store(rd, result as u64);

    Ok(cpu.pc + 4)
}