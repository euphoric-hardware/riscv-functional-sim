use crate::cpu::{Cpu, Insn};

pub fn srliw(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("srliw", rd = insn.rd(), rs1 = insn.rs1(), shamtw = insn.shamtw());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    todo!();
}