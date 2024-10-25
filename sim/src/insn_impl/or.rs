use crate::cpu::{Cpu, Insn};

pub fn or(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("or", rd = insn.rd(), rs1 = insn.rs1(), rs2 = insn.rs2());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let rs2 = insn.rs2();

    cpu.regs[rd as usize] = cpu.regs[rs1 as usize] | cpu.regs[rs2 as usize];
    cpu.pc += 4;
}