use crate::cpu::{Cpu, Insn};

pub fn srliw(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("srliw", rd = insn.rd(), rs1 = insn.rs1(), shamtw = insn.shamtw());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let shamtw = insn.shamtw();

    cpu.regs[rd as usize] = Insn::sign_extend((cpu.regs[rs1 as usize] as u32 >> shamtw) as u64, 32) as u64;
    cpu.pc += 4;
}