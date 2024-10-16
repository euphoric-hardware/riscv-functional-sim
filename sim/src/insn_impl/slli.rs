use crate::cpu::{Cpu, Insn};

pub fn slli(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("slli", rd = insn.rd(), rs1 = insn.rs1(), shamtd = insn.shamtd());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    todo!();
}