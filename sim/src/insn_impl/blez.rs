use crate::cpu::{Cpu, Insn};

pub fn blez(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("blez", bimm12hi = insn.bimm12hi(), rs2 = insn.rs2(), bimm12lo = insn.bimm12lo());

    let bimm12hi = insn.bimm12hi();
    let rs2 = insn.rs2();
    let bimm12lo = insn.bimm12lo();

    todo!();
}