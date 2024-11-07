use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn snez(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("snez", rd = insn.rd(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs2 = insn.rs2();

    todo!();
}