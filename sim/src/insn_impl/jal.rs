use crate::{cpu::{self, Cpu, Insn}, bus::Bus};

pub fn jal(insn: Insn, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
    crate::trace_insn!("jal", rd = insn.rd(), jimm20 = insn.jimm20());

    let rd = insn.rd();
    let jimm20 = insn.jimm20();

    todo!();
}