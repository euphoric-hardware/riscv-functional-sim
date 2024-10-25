use crate::cpu::{Cpu, Insn};

pub fn srai(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("srai", rd = insn.rd(), rs1 = insn.rs1(), shamtd = insn.shamtd());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtd = insn.shamtd();

    cpu.regs[rd as usize] = ((cpu.regs[rs1 as usize] as i64) >> shamtd) as u64;
    cpu.pc += 4;
}