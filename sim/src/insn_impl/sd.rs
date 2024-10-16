use crate::cpu::{Cpu, Insn};

pub fn sd(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("sd", imm12hi = insn.imm12hi(), rs1 = insn.rs1(), rs2 = insn.rs2(), imm12lo = insn.imm12lo());

    let imm12hi = insn.imm12hi();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();
    let imm12lo = insn.imm12lo();

    todo!();
}