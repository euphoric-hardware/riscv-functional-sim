use crate::cpu::{Cpu, Insn};

pub fn jal(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jal", rd = insn.rd(), jimm20 = insn.jimm20());

    let rd = insn.rd();
    let jimm20 = insn.jimm20();

    todo!();
}