use crate::cpu::{Cpu, Insn};

pub fn fence(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("fence", fm = insn.fm(), pred = insn.pred(), succ = insn.succ(), rs1 = insn.rs1(), rd = insn.rd());

    let fm = insn.fm();
    let pred = insn.pred();
    let succ = insn.succ();
    let rs1 = insn.rs1();
    let rd = insn.rd();

    todo!();
}