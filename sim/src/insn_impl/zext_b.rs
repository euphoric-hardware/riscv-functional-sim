use crate::cpu::{Cpu, Insn};

pub fn zext_b(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("zext_b", rd = insn.rd(), rs1 = insn.rs1());

    let rd = insn.rd();
    let rs1 = insn.rs1();

    todo!();
}