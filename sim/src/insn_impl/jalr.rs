use crate::cpu::{Cpu, Insn};

pub fn jalr(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jalr", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    todo!();
}