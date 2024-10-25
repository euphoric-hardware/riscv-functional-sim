use crate::cpu::{Cpu, Insn};

pub fn jalr(insn: Insn, cpu: &mut Cpu) {
    crate::trace_insn!("jalr", rd = insn.rd(), rs1 = insn.rs1(), imm12 = insn.imm12());

    let rd = insn.rd();
    let rs1 = insn.rs1();
    let imm12 = insn.imm12();

    let imm12_sign_extended = Insn::sign_extend(imm12 as u64,12);

    let stored_pc: u64 = cpu.pc + 4;
    let new_pc: u64 = cpu.regs[rs1 as usize] + imm12_sign_extended as u64;
    cpu.regs[rd as usize] = stored_pc;
    cpu.pc = new_pc;
}