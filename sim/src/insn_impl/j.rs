use crate::cpu::{Cpu, Insn};

pub fn j(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("j", jimm20 = insn.jimm20());

    let jimm20 = insn.jimm20();

    todo!();
}