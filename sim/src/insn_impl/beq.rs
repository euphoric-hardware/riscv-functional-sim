use crate::cpu::{Cpu, Insn};

pub fn beq(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("beq", bimm12hi = insn.bimm12hi(), rs1 = insn.rs1(), rs2 = insn.rs2(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    todo!();
}