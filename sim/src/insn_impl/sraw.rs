use crate::cpu::{Cpu, Insn};

pub fn sraw(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sraw", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    todo!();
}