use crate::cpu::{Cpu, Insn};

pub fn auipc(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("auipc", rd = insn.rd(), imm20 = insn.imm20());

    let rd = insn.rd();
    let imm20 = insn.imm20();

    todo!();
}