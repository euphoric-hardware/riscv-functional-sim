use crate::cpu::{Cpu, Insn};

pub fn andi(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("andi", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(20, 12);
    cpu.regs[rd as usize] = cpu.regs[rs1 as usize] & (imm12_sign_extended as u64);
    cpu.pc += 4;
}