use crate::cpu::{Cpu, Insn};

pub fn jr(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jr", rs1 = insn.rs1());

    let rs1 = insn.rs1();

    todo!();
}