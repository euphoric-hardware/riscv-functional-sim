use crate::cpu::{Cpu, Insn};

pub fn mv(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("mv", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    todo!();
}