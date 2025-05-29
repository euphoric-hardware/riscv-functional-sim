use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

#[inline(always)]
pub fn remu_raw(cpu: &mut Cpu, rd: u64, rs1: u64, rs2: u64) -> cpu::Result<u64> {
    let rs1_value = cpu.load(rs1);
    let rs2_value = cpu.load(rs2);

    if rs2_value == 0 {
        cpu.store(rd, rs1_value);
    } else {
        cpu.store(rd, rs1_value % rs2_value);
    }

    Ok(cpu.pc + 4)
}