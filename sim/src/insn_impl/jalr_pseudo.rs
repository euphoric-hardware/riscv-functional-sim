use crate::cpu::{Cpu, Insn};

pub fn jalr_pseudo(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jalr_pseudo", rs1 = insn.rs1());

    let rs1 = insn.rs1();

    todo!();
}