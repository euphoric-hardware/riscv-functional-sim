use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn divw_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let rs1_value = cpu.load(rs1) as u32 as i32;
    let rs2_value = cpu.load(rs2) as u32 as i32;

    if rs2_value == 0 {
        cpu.store(rd, u64::MAX);
    } else {
        cpu.store(rd, (rs1_value / rs2_value) as i64 as u64);
    }

    Ok(cpu.pc + 4)
}