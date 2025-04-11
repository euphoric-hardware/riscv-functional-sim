use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn c_addw_raw(cpu: &mut Cpu, rd_rs1_p: u64, rs2_p: u64) -> cpu::Result<u64> {
    let result = cpu.load(rd_rs1_p + 8).wrapping_add(cpu.load(rs2_p + 8)) as u32 as i32 as i64 as u64;
    cpu.store(rd_rs1_p, result);
    Ok(cpu.pc + 2)
}