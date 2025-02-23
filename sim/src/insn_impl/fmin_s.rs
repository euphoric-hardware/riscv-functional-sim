use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmin_s(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let value = f32::min(cpu.fload(rs1) as f32, cpu.fload(rs2) as f32) as f64;
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}