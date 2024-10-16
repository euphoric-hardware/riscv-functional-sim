use crate::cpu::{Cpu, Insn};

pub fn bgtu(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("bgtu", bimm12hi = insn.bimm12hi(), rs2 = insn.rs2(), rs1 = insn.rs1(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs2 = insn.rs2();
    let rs1 = insn.rs1();
    let bimm12lo = insn.bimm12lo();

    todo!();
}