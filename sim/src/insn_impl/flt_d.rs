use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn flt_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    let value = if f64::from_bits(*cpu.fload(rs1).bits()) < f64::from_bits(*cpu.fload(rs2).bits()) {
        1
    } else {
        0
    };
    cpu.store(rd, value);
    Ok(cpu.pc + 4)
}