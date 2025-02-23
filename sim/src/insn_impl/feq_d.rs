use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn feq_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    todo!();
}