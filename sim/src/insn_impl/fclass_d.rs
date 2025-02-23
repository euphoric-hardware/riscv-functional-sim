use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn fclass_d(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("fclass_d", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    todo!();
}