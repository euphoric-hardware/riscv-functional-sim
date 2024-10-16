use crate::cpu::{Cpu, Insn};

pub fn fence_tso(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("fence_tso", rs1 = insn.rs1(), rd = insn.rd());

    let rs1 = insn.rs1();
    let rd = insn.rd();

    todo!();
}