use crate::cpu::{Cpu, Insn};

pub fn sext_w(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sext_w", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    todo!();
}