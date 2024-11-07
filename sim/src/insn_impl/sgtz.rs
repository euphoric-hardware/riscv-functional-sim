use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn sgtz(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("sgtz", rd = insn.rd(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs2 = insn.rs2();

    todo!();
}