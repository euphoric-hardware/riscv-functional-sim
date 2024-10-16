use crate::cpu::{Cpu, Insn};

pub fn snez(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("snez", rd = insn.rd(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs2 = insn.rs2();

    todo!();
}