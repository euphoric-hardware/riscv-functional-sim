use crate::cpu::{Cpu, Insn};

pub fn jal_pseudo(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jal_pseudo", jimm20 = insn.jimm20());

    let jimm20 = insn.jimm20();

    todo!();
}