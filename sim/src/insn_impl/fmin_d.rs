use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fmin_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let value = f64::min(cpu.fload(rs1), cpu.fload(rs2));
    cpu.fstore(rd, value);
    Ok(cpu.pc + 4)
}