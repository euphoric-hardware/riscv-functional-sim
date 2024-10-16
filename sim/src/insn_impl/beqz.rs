use crate::cpu::{Cpu, Insn};

pub fn beqz(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("beqz", bimm12hi = insn.bimm12hi(), rs1 = insn.rs1(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let bimm12lo = insn.bimm12lo();

    todo!();
}