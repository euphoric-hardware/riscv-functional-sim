use crate::cpu::{Cpu, Insn};

pub fn add(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("add", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    todo!();
}